import type { EditorState } from "prosemirror-state"
import { schema } from "../components/editor/prosemirror/schema"

export const isEmptyTopLevelParagraph = (state: EditorState): boolean => {
    const { selection } = state
    const { $from } = selection
    return selection.empty && $from.depth === 1 && $from.parent.type === schema.nodes.paragraph && $from.parent.content.size === 0
}
