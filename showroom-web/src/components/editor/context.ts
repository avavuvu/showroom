import { context } from "../../../bindings/boutique/setups"
import type { Command } from "prosemirror-state"
import type { EditorView } from "prosemirror-view"
import type { ReviewSummary } from "../../../bindings/types/ReviewSummary"

export interface Editor {
    view: EditorView
    onUpdate(listener: (view: EditorView) => void, signal: AbortSignal): void

    insertImage: Command
    reviews?: {
        summaries: ReadonlyArray<ReviewSummary>
        sendUrl: (sectionId: string) => string
        listUrl: string
        navigate: (href: string) => void
    }
}

export const editorContext = context<Editor>("editor")
