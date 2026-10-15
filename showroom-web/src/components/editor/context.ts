import { context } from "@bq/components/src/setups";
import type { Command } from "prosemirror-state";
import type { EditorView } from "prosemirror-view";

export interface Editor {
    view: EditorView;
    onUpdate(listener: (view: EditorView) => void, signal: AbortSignal): void;

    insertImage: Command;
}

export const editorContext = context<Editor>("editor");
