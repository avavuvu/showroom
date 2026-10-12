import { context } from "@bq/components/src/setups";
import type { EditorView } from "prosemirror-view";

export interface Editor {
    view: EditorView;
    onUpdate(listener: (view: EditorView) => void, signal: AbortSignal): void;
    report(message: string): void;
}

export const editorContext = context<Editor>("editor");
