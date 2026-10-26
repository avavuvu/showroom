import { chainCommands, exitCode, splitBlockAs, toggleMark } from "prosemirror-commands"
import { redo, undo } from "prosemirror-history"
import { undoInputRule } from "prosemirror-inputrules"
import { Fragment, type Node, NodeRange } from "prosemirror-model"
import { liftListItem, sinkListItem, splitListItem } from "prosemirror-schema-list"
import { type Command, NodeSelection, Selection, TextSelection } from "prosemirror-state"
import { openLink } from "../link_bar/link"
import { BLOCKS, canInsert, findAncestor, insertHorizontalRule, toggleBlockquote, toggleList } from "./commands"
import { toggleBoxout } from "../../../blocks/boxout/commands"
import { deleteNameBackward, deleteNameForward, editName } from "./name"
import { schema } from "./schema"

const { paragraph, pullquote, attribution, bulletList, orderedList, listItem, hardBreak, caption } = schema.nodes

const insertHardBreak: Command = (state, dispatch) => {
    if (!canInsert(state, hardBreak)) {
        return false
    }
    if (dispatch) {
        dispatch(state.tr.replaceSelectionWith(hardBreak.create()).scrollIntoView())
    }
    return true
}

const isInList: Command = (state) => !!findAncestor(state, (node) => node.type === listItem)

const exitCaption: Command = (state, dispatch) => {
    const { $from } = state.selection
    if ($from.parent.type !== caption && $from.parent.type !== attribution) {
        return false
    }
    if (!dispatch) {
        return true
    }

    const after = $from.after($from.depth - 1)
    const next = state.doc.resolve(after).nodeAfter
    const transaction = state.tr
    if (next?.isTextblock) {
        transaction.setSelection(TextSelection.create(transaction.doc, after + 1))
    } else {
        transaction.insert(after, paragraph.create())
        transaction.setSelection(TextSelection.create(transaction.doc, after + 1))
    }
    dispatch(transaction.scrollIntoView())
    return true
}

const enterSource: Command = (state, dispatch) => {
    const { $from, empty } = state.selection
    if (!empty || $from.depth < 2 || $from.parent.type !== paragraph || $from.parent.content.size !== 0) {
        return false
    }
    const parent = $from.node($from.depth - 1)
    if (parent.type !== pullquote || $from.index($from.depth - 1) !== parent.childCount - 2) {
        return false
    }
    if (!dispatch) {
        return true
    }

    const start = $from.before()
    const transaction = state.tr
    if (parent.childCount > 2) {
        transaction.delete(start, $from.after())
    }
    dispatch(transaction.setSelection(TextSelection.create(transaction.doc, parent.childCount > 2 ? start + 1 : $from.after() + 1)).scrollIntoView())
    return true
}

const moveBlock =
    (direction: -1 | 1): Command =>
        (state, dispatch) => {
            const { selection } = state
            let range = selection.$from.blockRange(selection.$to)

            while (range) {
                const { parent, startIndex, endIndex } = range
                const swapIndex = direction < 0 ? startIndex - 1 : endIndex

                if (swapIndex >= 0 && swapIndex < parent.childCount) {
                    const sibling = parent.child(swapIndex)
                    const moved: Array<Node> = []
                    for (let index = startIndex; index < endIndex; index++) {
                        moved.push(parent.child(index))
                    }

                    const children: Array<Node> = []
                    parent.forEach((child) => children.push(child))
                    const first = Math.min(swapIndex, startIndex)
                    const reordered = direction < 0 ? [...moved, sibling] : [sibling, ...moved]
                    children.splice(first, reordered.length, ...reordered)

                    if (parent.type.validContent(Fragment.fromArray(children))) {
                        if (dispatch) {
                            const shift = direction < 0 ? -sibling.nodeSize : sibling.nodeSize
                            const from = direction < 0 ? range.start - sibling.nodeSize : range.start
                            const to = direction < 0 ? range.end : range.end + sibling.nodeSize
                            const transaction = state.tr.replaceWith(from, to, reordered)

                            let newSelection
                            if (selection instanceof NodeSelection) {
                                newSelection = NodeSelection.create(transaction.doc, selection.from + shift)
                            } else if (selection instanceof TextSelection) {
                                newSelection = TextSelection.create(transaction.doc, selection.anchor + shift, selection.head + shift)
                            } else {
                                newSelection = Selection.near(transaction.doc.resolve(selection.from + shift))
                            }

                            transaction.setSelection(newSelection)
                            dispatch(transaction.scrollIntoView())
                        }
                        return true
                    }
                }

                if (range.depth === 0) {
                    return false
                }
                const $start = state.doc.resolve(range.$from.before(range.depth))
                const $end = state.doc.resolve(range.$from.after(range.depth))
                range = new NodeRange($start, $end, range.depth - 1)
            }
            return false
        }

const insertLine =
    (direction: -1 | 1): Command =>
        (state, dispatch) => {
            const { selection } = state
            const { $from } = selection
            if (direction > 0 && $from.parent.type.spec.code) {
                return exitCode(state, dispatch)
            }

            const insert = (pos: number, node: Node, cursor: number) => {
                if (dispatch) {
                    const transaction = state.tr.insert(pos, node)
                    dispatch(transaction.setSelection(TextSelection.create(transaction.doc, pos + cursor)).scrollIntoView())
                }
                return true
            }

            if (selection instanceof NodeSelection) {
                const index = direction > 0 ? $from.index() + 1 : $from.index()
                if ($from.parent.canReplaceWith(index, index, paragraph)) {
                    return insert(direction > 0 ? selection.to : selection.from, paragraph.create(), 1)
                }
            }

            for (let depth = $from.depth; depth > 0; depth--) {
                const container = $from.node(depth - 1)
                const index = direction > 0 ? $from.indexAfter(depth - 1) : $from.index(depth - 1)
                const pos = direction > 0 ? $from.after(depth) : $from.before(depth)

                if (container.type === listItem && depth > 1) {
                    continue
                }
                if (container.type === bulletList || container.type === orderedList) {
                    return insert(pos, listItem.create(null, paragraph.create()), 2)
                }
                if (container.canReplaceWith(index, index, paragraph)) {
                    return insert(pos, paragraph.create(), 1)
                }
            }
            return false
        }

const enterCaption: Command = (state, dispatch) => {
    const { selection } = state
    if (!(selection instanceof NodeSelection) || selection.node.type !== schema.nodes.figure) {
        return false
    }
    if (dispatch) {
        const end = selection.from + selection.node.nodeSize - 2
        dispatch(state.tr.setSelection(TextSelection.create(state.doc, end)).scrollIntoView())
    }
    return true
}

const splitInPullquote: Command = (state, dispatch) => {
    const { $from } = state.selection
    if ($from.depth < 2 || $from.node(-1).type !== pullquote || $from.parent.content.size === 0) {
        return false
    }
    return splitBlockAs((_node, isAtEnd) => (isAtEnd ? { type: paragraph } : null))(state, dispatch)
}

const selectFigure: Command = (state, dispatch) => {
    const { $from } = state.selection
    if ($from.parent.type !== caption) {
        return false
    }
    if (dispatch) {
        dispatch(state.tr.setSelection(NodeSelection.create(state.doc, $from.before($from.depth - 1))))
    }
    return true
}

export const buildKeymap = (): Record<string, Command> => {
    const { bold, italic, underline, strike, code } = schema.marks
    return {
        "Mod-z": undo,
        "Shift-Mod-z": redo,
        "Mod-y": redo,
        Backspace: chainCommands(undoInputRule, deleteNameBackward),
        Delete: deleteNameForward,
        "Mod-b": toggleMark(bold),
        "Mod-i": toggleMark(italic),
        "Mod-u": toggleMark(underline),
        "Shift-Mod-x": toggleMark(strike),
        "Mod-e": toggleMark(code),
        "Mod-k": (state, dispatch, view) => {
            chainCommands(editName, openLink)(state, dispatch, view)
            return true
        },

        "Mod-Alt-0": BLOCKS.paragraph,
        "Mod-Alt-2": BLOCKS["heading-2"],
        "Mod-Alt-3": BLOCKS["heading-3"],
        "Mod-Alt-4": BLOCKS["heading-4"],
        "Mod-Alt-c": BLOCKS["code-block"],
        "Shift-Mod-7": toggleList(orderedList),
        "Shift-Mod-8": toggleList(bulletList),
        "Shift-Mod-9": toggleBlockquote,
        "Shift-Mod-0": toggleBoxout,
        "Mod-Alt--": insertHorizontalRule,
        Enter: chainCommands(enterCaption, exitCaption, enterSource, splitInPullquote, splitListItem(listItem)),
        Escape: selectFigure,
        "Mod-Enter": insertLine(1),
        "Shift-Mod-Enter": insertLine(-1),
        "Alt-ArrowUp": moveBlock(-1),
        "Alt-ArrowDown": moveBlock(1),
        "Shift-Enter": chainCommands(exitCode, insertHardBreak),
        Tab: chainCommands(sinkListItem(listItem), isInList),
        "Shift-Tab": chainCommands(liftListItem(listItem), isInList),
    }
}
