export type SaveState = "saved" | "dirty" | "saving" | "error" | "conflict" | "expired" | "invalid";

export interface SavePayload {
    title: string;
    subtitle: string | null;
    content: unknown;
}

interface SaveOptions {
    url: string;
    revision: number;
    read: () => SavePayload;
    onChange: (state: SaveState) => void;
    signal: AbortSignal;
}

type Outcome =
    | { kind: "ok"; revision: number }
    | { kind: "conflict"; revision: number }
    | { kind: "expired" }
    | { kind: "invalid" }
    | { kind: "failed" };

const DEBOUNCE_MS = 1_500;
const MAX_WAIT_MS = 10_000;
const RETRY_MS = [2_000, 5_000, 15_000, 30_000];

export class SaveController {
    state: SaveState = "saved";

    private revision: number;
    private serverRevision: number | null = null;
    private edits = 0;
    private savedEdits = 0;
    private retries = 0;
    private discarded = false;
    private queue: Promise<unknown> = Promise.resolve();
    private debounceTimer: ReturnType<typeof setTimeout> | undefined;
    private maxWaitTimer: ReturnType<typeof setTimeout> | undefined;
    private retryTimer: ReturnType<typeof setTimeout> | undefined;

    constructor(private options: SaveOptions) {
        this.revision = options.revision;
        window.addEventListener("online", () => this.state === "error" && void this.save(false), { signal: options.signal });
        options.signal.addEventListener("abort", () => this.clearTimers(), { once: true });
    }

    get unsaved(): boolean {
        return !this.discarded && this.edits !== this.savedEdits;
    }

    markDirty() {
        this.edits++;
        if (this.state === "saved") this.set("dirty");
        if (!this.blocked(false)) this.schedule();
    }

    flush = async (): Promise<boolean> => {
        for (let attempt = 0; attempt < 5 && this.unsaved; attempt++) {
            if (!(await this.save(true))) return false;
        }
        return !this.unsaved;
    };

    overwrite(): Promise<boolean> {
        if (this.serverRevision != null) this.revision = this.serverRevision;
        this.serverRevision = null;
        this.set("dirty");
        return this.flush();
    }

    discard() {
        this.discarded = true;
        this.clearTimers();
    }

    private blocked(manual: boolean): boolean {
        if (this.state === "conflict") return true;
        return !manual && (this.state === "expired" || this.state === "invalid");
    }

    private schedule() {
        clearTimeout(this.debounceTimer);
        this.debounceTimer = setTimeout(() => void this.save(false), DEBOUNCE_MS);
        this.maxWaitTimer ??= setTimeout(() => void this.save(false), MAX_WAIT_MS);
    }

    private clearTimers() {
        clearTimeout(this.debounceTimer);
        clearTimeout(this.maxWaitTimer);
        clearTimeout(this.retryTimer);
        this.debounceTimer = this.maxWaitTimer = this.retryTimer = undefined;
    }

    private save(manual: boolean): Promise<boolean> {
        this.clearTimers();
        const run = this.queue.then(() => this.attempt(manual));
        this.queue = run.catch(() => undefined);
        return run;
    }

    private async attempt(manual: boolean): Promise<boolean> {
        if (this.options.signal.aborted || this.discarded) return false;
        if (!this.unsaved) {
            if (!this.blocked(true)) this.set("saved");
            return true;
        }
        if (this.blocked(manual)) return false;

        const edits = this.edits;
        this.set("saving");
        const outcome = await this.send(this.options.read());

        switch (outcome.kind) {
            case "ok":
                this.revision = outcome.revision;
                this.savedEdits = edits;
                this.retries = 0;
                if (this.unsaved) {
                    this.set("dirty");
                    this.schedule();
                } else {
                    this.set("saved");
                }
                return true;
            case "conflict":
                this.serverRevision = outcome.revision;
                this.set("conflict");
                return false;
            case "expired":
                this.set("expired");
                return false;
            case "invalid":
                this.set("invalid");
                return false;
            case "failed":
                this.set("error");
                this.retryTimer = setTimeout(() => void this.save(false), RETRY_MS[Math.min(this.retries, RETRY_MS.length - 1)]);
                this.retries++;
                return false;
        }
    }

    private async send(payload: SavePayload): Promise<Outcome> {
        try {
            const response = await fetch(this.options.url, {
                method: "PUT",
                redirect: "manual",
                headers: { "Content-Type": "application/json", Accept: "application/json" },
                body: JSON.stringify({ ...payload, revision: this.revision }),
            });

            if (response.type === "opaqueredirect" || response.status === 401 || response.status === 403) return { kind: "expired" };
            if (response.status === 409) {
                const body = (await response.json()) as { revision: number };
                return { kind: "conflict", revision: Number(body.revision) };
            }
            if ([400, 413, 415, 422].includes(response.status)) return { kind: "invalid" };
            if (!response.ok) return { kind: "failed" };

            const body = (await response.json()) as { revision: number };
            return { kind: "ok", revision: Number(body.revision) };
        } catch {
            return { kind: "failed" };
        }
    }

    private set(state: SaveState) {
        if (this.state === state) return;
        this.state = state;
        this.options.onChange(state);
    }
}
