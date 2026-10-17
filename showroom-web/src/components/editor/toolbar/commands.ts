import { toggleMark } from "prosemirror-commands"
import { redo, undo } from "prosemirror-history"
import type { NodeType } from "prosemirror-model"
import type { Command, EditorState } from "prosemirror-state"
import { findAncestor, insertHorizontalRule, isList, isMarkActive, toggleBlockquote, toggleList, togglePullquote } from "../prosemirror/commands"
import { openLink } from "../link_bar/link"
import { schema } from "../prosemirror/schema"

export interface ToolbarCommand {
    run: Command
    isActive?: (state: EditorState) => boolean
}

const { blockquote, pullquote, bulletList, orderedList } = schema.nodes

export const createToolbarCommands = (insertImage: Command): Record<string, ToolbarCommand> => {
    const { bold, italic, underline, strike, code, link } = schema.marks
    const getListType = (state: EditorState) => findAncestor(state, isList)?.node.type ?? null
    const checkInside = (type: NodeType) => (state: EditorState) => !!findAncestor(state, (node) => node.type === type)

    return {
        undo: { run: undo },
        redo: { run: redo },
        bold: { run: toggleMark(bold), isActive: (state) => isMarkActive(state, bold) },
        italic: { run: toggleMark(italic), isActive: (state) => isMarkActive(state, italic) },
        underline: { run: toggleMark(underline), isActive: (state) => isMarkActive(state, underline) },
        strike: { run: toggleMark(strike), isActive: (state) => isMarkActive(state, strike) },
        code: { run: toggleMark(code), isActive: (state) => isMarkActive(state, code) },
        link: { run: openLink, isActive: (state) => isMarkActive(state, link) },
        "bullet-list": { run: toggleList(bulletList), isActive: (state) => getListType(state) === bulletList },
        "ordered-list": { run: toggleList(orderedList), isActive: (state) => getListType(state) === orderedList },
        blockquote: { run: toggleBlockquote, isActive: checkInside(blockquote) },
        pullquote: { run: togglePullquote, isActive: checkInside(pullquote) },
        "horizontal-rule": { run: insertHorizontalRule },
        image: { run: insertImage },
    }
}
