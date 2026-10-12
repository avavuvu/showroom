import type { SaveState } from "../save";
import { formatShortcut } from "../shortcuts";

const ALERT_STATES: SaveState[] = ["error", "conflict", "expired", "invalid"];
const PROBLEM_MS = 8_000;

const messages = (saveKeys: string): Record<SaveState, string> => ({
    saved: "Saved",
    dirty: "Unsaved changes",
    saving: "Saving…",
    error: "Could not save. The editor will try again.",
    conflict: "This newsletter changed in another tab or window.",
    expired: `Your session ended. Log in again in a new tab, then press ${saveKeys}.`,
    invalid: "The server did not accept this content. Undo your last change, then try again.",
});

export class SaveStatus {
    private text = messages(formatShortcut("Mod-s"));
    private timer: ReturnType<typeof setTimeout> | undefined;

    constructor(
        private output: HTMLElement,
        private actions: HTMLElement,
    ) {}

    set(state: string, message: string, alert: boolean) {
        if (alert) this.output.setAttribute("role", "alert");
        else this.output.removeAttribute("role");
        this.output.dataset.state = state;
        this.output.textContent = message;
    }

    render(state: SaveState) {
        clearTimeout(this.timer);
        this.set(state, this.text[state], ALERT_STATES.includes(state));
        this.actions.hidden = state !== "conflict";
    }

    problem(message: string, current: () => SaveState) {
        this.set("problem", message, true);
        clearTimeout(this.timer);
        this.timer = setTimeout(() => this.render(current()), PROBLEM_MS);
    }

    stop() {
        clearTimeout(this.timer);
    }
}
