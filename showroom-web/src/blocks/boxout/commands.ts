import type { Node } from "prosemirror-model"
import { type Command, type EditorState, NodeSelection, Plugin, type Transaction } from "prosemirror-state"
import { findWrapping, liftTarget } from "prosemirror-transform"
import { Decoration, DecorationSet } from "prosemirror-view"
import { schema } from "../../components/editor/prosemirror/schema"

interface FoundBoxout {
    node: Node
    pos: number
}

export const findBoxout = (state: EditorState): FoundBoxout | null => {
    const { selection } = state
    if (selection instanceof NodeSelection && selection.node.type === schema.nodes.boxout) {
        return { node: selection.node, pos: selection.from }
    }
    const { $from } = selection
    for (let depth = $from.depth; depth > 0; depth--) {
        const node = $from.node(depth)
        if (node.type === schema.nodes.boxout) {
            return { node, pos: $from.before(depth) }
        }
    }
    return null
}

export const isInBoxout = (state: EditorState): boolean => !!findBoxout(state)

export const unwrapBoxoutAt = (transaction: Transaction, pos: number): boolean => {
    const node = transaction.doc.nodeAt(pos)
    if (node?.type !== schema.nodes.boxout) {
        return false
    }
    const $start = transaction.doc.resolve(pos + 1)
    const $end = transaction.doc.resolve(pos + node.nodeSize - 1)
    const range = $start.blockRange($end)
    const target = range && liftTarget(range)
    if (!range || target == null) {
        return false
    }
    transaction.lift(range, target)
    return true
}

export const toggleBoxout: Command = (state, dispatch) => {
    const found = findBoxout(state)
    if (found) {
        const transaction = state.tr
        if (!unwrapBoxoutAt(transaction, found.pos)) {
            return false
        }
        dispatch?.(transaction.scrollIntoView())
        return true
    }

    const { $from, $to } = state.selection
    const range = $from.blockRange($to, (node) => node.type === schema.nodes.doc)
    const wrapping = range && findWrapping(range, schema.nodes.boxout)
    if (!range || !wrapping) {
        return false
    }
    dispatch?.(state.tr.wrap(range, wrapping).scrollIntoView())
    return true
}

export const boxoutPlugin = () => new Plugin({
    props: {
        decorations: (state) => {
            const found = findBoxout(state)
            if (!found) {
                return null
            }
            return DecorationSet.create(state.doc, [
                Decoration.node(found.pos, found.pos + found.node.nodeSize, { class: "has-cursor" }),
            ])
        },
    },
})
