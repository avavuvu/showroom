import type { SaveState } from "../save"
import { formatShortcut } from "../shortcuts"

const PROBLEM_STATES: Array<SaveState> = ["error", "conflict", "expired", "invalid"]
const PROBLEM_MS = 8_000

const createMessages = (saveKeys: string): Record<SaveState, string> => ({
    saved: "Saved",
    dirty: "Unsaved changes",
    saving: "Saving…",
    error: "Could not save. The editor will try again.",
    conflict: "This newsletter changed in another tab or window.",
    expired: `Your session ended. Log in again in a new tab, then press ${saveKeys}.`,
    invalid: "The server did not accept this content. Undo your last change, then try again.",
})

interface Regions {
    status: HTMLElement
    announce: HTMLElement
    alert: HTMLElement
    actions: HTMLElement
}

export class SaveStatus {
    private messages: Record<SaveState, string>
    private timer: ReturnType<typeof setTimeout> | undefined

    constructor(private regions: Regions, overrides: Partial<Record<SaveState, string>> = {}) {
        this.messages = { ...createMessages(formatShortcut("Mod-s")), ...overrides }
    }

    render(state: SaveState) {
        clearTimeout(this.timer)
        this.show(state, this.messages[state])
        this.regions.actions.hidden = state !== "conflict"

        if (PROBLEM_STATES.includes(state)) {
            this.regions.announce.textContent = ""
            this.regions.alert.textContent = this.messages[state]
        } else if (state === "saved") {
            this.regions.alert.textContent = ""
            if (this.regions.announce.textContent !== this.messages.saved) {
                this.regions.announce.textContent = this.messages.saved
            }
        }
    }

    showProblem(message: string, getCurrentState: () => SaveState) {
        clearTimeout(this.timer)
        this.show("problem", message)
        this.regions.alert.textContent = message
        this.timer = setTimeout(() => this.render(getCurrentState()), PROBLEM_MS)
    }

    showFailure(message: string) {
        clearTimeout(this.timer)
        this.show("error", message)
        this.regions.announce.textContent = ""
        this.regions.alert.textContent = message
    }

    stop() {
        clearTimeout(this.timer)
    }

    private show(state: string, message: string) {
        this.regions.status.dataset.state = state
        this.regions.status.textContent = message
    }
}
