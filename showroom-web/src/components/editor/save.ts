import type { SaveRequest } from "../../../bindings/types/SaveRequest"
import type { SaveResponse } from "../../../bindings/types/SaveResponse"

export type SaveState = "saved" | "dirty" | "saving" | "error" | "conflict" | "expired" | "invalid"

export type SavePayload = Omit<SaveRequest, "revision">

interface SaveOptions {
    url: string
    revision: number
    read: () => SavePayload
    onChange: (state: SaveState) => void
    signal: AbortSignal
}

type Outcome =
    | { kind: "ok", revision: number }
    | { kind: "conflict", revision: number }
    | { kind: "expired" }
    | { kind: "invalid" }
    | { kind: "failed" }

const DEBOUNCE_MS = 1_500
const MAX_WAIT_MS = 10_000
const RETRY_MS = [2_000, 5_000, 15_000, 30_000]

export class SaveController {
    state: SaveState = "saved"

    private revision: number
    private serverRevision: number | null = null
    private edits = 0
    private savedEdits = 0
    private retries = 0
    private isDiscarded = false
    private queue: Promise<unknown> = Promise.resolve()
    private debounceTimer: ReturnType<typeof setTimeout> | undefined
    private maxWaitTimer: ReturnType<typeof setTimeout> | undefined
    private retryTimer: ReturnType<typeof setTimeout> | undefined

    constructor(private options: SaveOptions) {
        this.revision = options.revision
        window.addEventListener("online", () => this.state === "error" && void this.save(false), { signal: options.signal })
        options.signal.addEventListener("abort", () => this.clearTimers(), { once: true })
    }

    stop() {
        this.clearTimers()
    }

    get hasUnsavedChanges(): boolean {
        return !this.isDiscarded && this.edits !== this.savedEdits
    }

    markDirty() {
        this.edits++
        if (this.state === "saved") {
            this.setState("dirty")
        }
        if (!this.isBlocked(false)) {
            this.schedule()
        }
    }

    flush = async (): Promise<boolean> => {
        for (let attempt = 0; attempt < 5 && this.hasUnsavedChanges; attempt++) {
            if (!(await this.save(true))) {
                return false
            }
        }
        return !this.hasUnsavedChanges
    }

    overwrite(): Promise<boolean> {
        if (this.serverRevision != null) {
            this.revision = this.serverRevision
        }
        this.serverRevision = null
        this.setState("dirty")
        return this.flush()
    }

    discard() {
        this.isDiscarded = true
        this.clearTimers()
    }

    private isBlocked(isManual: boolean): boolean {
        if (this.state === "conflict") {
            return true
        }
        return !isManual && (this.state === "expired" || this.state === "invalid")
    }

    private schedule() {
        if (this.options.signal.aborted) {
            return
        }
        clearTimeout(this.debounceTimer)
        this.debounceTimer = setTimeout(() => void this.save(false), DEBOUNCE_MS)
        this.maxWaitTimer ??= setTimeout(() => void this.save(false), MAX_WAIT_MS)
    }

    private clearTimers() {
        clearTimeout(this.debounceTimer)
        clearTimeout(this.maxWaitTimer)
        clearTimeout(this.retryTimer)
        this.debounceTimer = this.maxWaitTimer = this.retryTimer = undefined
    }

    private save(isManual: boolean): Promise<boolean> {
        this.clearTimers()
        const result = this.queue.then(() => this.attempt(isManual))
        this.queue = result.catch(() => undefined)
        return result
    }

    private async attempt(isManual: boolean): Promise<boolean> {
        if (this.options.signal.aborted || this.isDiscarded) {
            return false
        }
        if (!this.hasUnsavedChanges) {
            if (!this.isBlocked(true)) {
                this.setState("saved")
            }
            return true
        }
        if (this.isBlocked(isManual)) {
            return false
        }

        const edits = this.edits
        this.setState("saving")
        const outcome = await this.send(this.options.read())
        if (this.options.signal.aborted) {
            return false
        }

        switch (outcome.kind) {
            case "ok":
                this.revision = outcome.revision
                this.savedEdits = edits
                this.retries = 0
                if (this.hasUnsavedChanges) {
                    this.setState("dirty")
                    this.schedule()
                } else {
                    this.setState("saved")
                }
                return true
            case "conflict":
                this.serverRevision = outcome.revision
                this.setState("conflict")
                return false
            case "expired":
                this.setState("expired")
                return false
            case "invalid":
                this.setState("invalid")
                return false
            case "failed":
                this.setState("error")
                this.retryTimer = setTimeout(() => void this.save(false), RETRY_MS[Math.min(this.retries, RETRY_MS.length - 1)])
                this.retries++
                return false
        }
    }

    private async send(payload: SavePayload): Promise<Outcome> {
        try {
            const response = await fetch(this.options.url, {
                method: "PUT",
                redirect: "manual",
                signal: this.options.signal,
                headers: { "Content-Type": "application/json", Accept: "application/json" },
                body: JSON.stringify({ ...payload, revision: this.revision } satisfies SaveRequest),
            })

            if (response.type === "opaqueredirect" || response.status === 401 || response.status === 403) {
                return { kind: "expired" }
            }
            if (response.status === 409) {
                const body = (await response.json()) as SaveResponse
                return { kind: "conflict", revision: Number(body.revision) }
            }
            if ([400, 413, 415, 422].includes(response.status)) {
                return { kind: "invalid" }
            }
            if (!response.ok) {
                return { kind: "failed" }
            }

            const body = (await response.json()) as SaveResponse
            return { kind: "ok", revision: Number(body.revision) }
        } catch {
            return { kind: "failed" }
        }
    }

    private setState(state: SaveState) {
        if (this.state === state) {
            return
        }
        this.state = state
        this.options.onChange(state)
    }
}
