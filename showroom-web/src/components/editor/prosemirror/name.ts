import type { Node } from "prosemirror-model"
import { type Command, type EditorState, NodeSelection, Plugin, PluginKey, Selection, TextSelection, type Transaction } from "prosemirror-state"
import type { EditorView } from "prosemirror-view"
import { NAME_TEXT, schema } from "./schema"

const NAME_PATTERN = /\{\{\s*name\s*\}\}/gi
const OTHER_INLINE = "\uFFFC"

const { name } = schema.nodes
const { code } = schema.marks

const getChangedRanges = (transactions: ReadonlyArray<Transaction>): Array<[number, number]> => {
    let ranges: Array<[number, number]> = []
    for (const transaction of transactions) {
        for (const map of transaction.mapping.maps) {
            ranges = ranges.map(([from, to]) => [map.map(from, -1), map.map(to, 1)])
            map.forEach((_oldStart, _oldEnd, newStart, newEnd) => ranges.push([newStart, newEnd]))
        }
    }
    return ranges
}

const getBlockText = (block: Node): string => {
    let text = ""
    block.forEach((child) => {
        text += child.isText ? (child.text ?? "") : OTHER_INLINE.repeat(child.nodeSize)
    })
    return text
}

const convertTextToNames = (transactions: ReadonlyArray<Transaction>, doc: Node, transaction: Transaction): boolean => {
    const blocks = new Map<number, Node>()
    for (const [from, to] of getChangedRanges(transactions)) {
        doc.nodesBetween(Math.max(0, from - 1), Math.min(doc.content.size, to + 1), (node, pos) => {
            if (!node.isTextblock) {
                return true
            }
            if (!node.type.spec.code && node.type.contentMatch.matchType(name)) {
                blocks.set(pos, node)
            }
            return false
        })
    }

    const replacements: Array<{ from: number, to: number }> = []
    for (const [pos, block] of blocks) {
        const start = pos + 1
        for (const match of getBlockText(block).matchAll(NAME_PATTERN)) {
            const from = start + match.index
            const to = from + match[0].length
            if (!doc.rangeHasMark(from, to, code)) {
                replacements.push({ from, to })
            }
        }
    }

    replacements.sort((a, b) => b.from - a.from)
    for (const { from, to } of replacements) {
        const marks = doc.nodeAt(from)?.marks ?? []
        transaction.replaceWith(from, to, name.create(null, null, marks))
    }
    return replacements.length > 0
}

const nameEditKey = new PluginKey<boolean>("name-edit")

export interface SelectedName {
    pos: number
    fallback: string
}

export const getSelectedName = (state: EditorState): SelectedName | null => {
    const { selection } = state
    if (!(selection instanceof NodeSelection) || selection.node.type !== name) {
        return null
    }
    return { pos: selection.from, fallback: selection.node.attrs.fallback as string }
}

export const isEditingName = (state: EditorState): boolean => nameEditKey.getState(state) ?? false

export const editName: Command = (state, dispatch) => {
    if (!getSelectedName(state)) {
        return false
    }
    dispatch?.(state.tr.setMeta(nameEditKey, true))
    return true
}

export const setNameFallback = (view: EditorView, fallback: string) => {
    const selected = getSelectedName(view.state)
    if (!selected || selected.fallback === fallback) {
        return
    }
    const transaction = view.state.tr.setNodeAttribute(selected.pos, "fallback", fallback)
    view.dispatch(transaction.setSelection(NodeSelection.create(transaction.doc, selected.pos)))
}

export const stopEditingName = (view: EditorView) => {
    if (!view.isDestroyed && isEditingName(view.state)) {
        view.dispatch(view.state.tr.setMeta(nameEditKey, false))
    }
}

export const namePlugin = () => new Plugin<boolean>({
    key: nameEditKey,
    state: {
        init: () => false,
        apply: (transaction, isEditing, _previous, state) => {
            const isRequested = transaction.getMeta(nameEditKey) as boolean | undefined
            return isRequested ?? (isEditing && !!getSelectedName(state))
        },
    },
    props: {
        handleDoubleClickOn: (view, _pos, node, nodePos) => {
            if (node.type !== name) {
                return false
            }
            const transaction = view.state.tr.setSelection(NodeSelection.create(view.state.doc, nodePos))
            view.dispatch(transaction.setMeta(nameEditKey, true))
            return true
        },
    },
    appendTransaction: (transactions, _previous, state) => {
        if (!transactions.some((transaction) => transaction.docChanged)) {
            return null
        }
        const transaction = state.tr
        return convertTextToNames(transactions, state.doc, transaction) ? transaction : null
    },
})

export const convertNamesToText = (transaction: Transaction, from: number, to: number) => {
    const tokens: Array<number> = []
    transaction.doc.nodesBetween(from, to, (node, pos) => {
        if (!node.isTextblock) {
            return true
        }
        node.forEach((child, offset) => {
            if (child.type === name) {
                tokens.push(pos + 1 + offset)
            }
        })
        return false
    })
    for (const pos of tokens.reverse()) {
        transaction.replaceWith(pos, pos + 1, schema.text(NAME_TEXT))
    }
}

const deleteName =
    (direction: -1 | 1): Command =>
        (state, dispatch) => {
            const { selection } = state
            if (!(selection instanceof TextSelection) || !selection.empty) {
                return false
            }
            const node = direction < 0 ? selection.$from.nodeBefore : selection.$from.nodeAfter
            if (node?.type !== name) {
                return false
            }
            if (dispatch) {
                const from = direction < 0 ? selection.from - 1 : selection.from
                dispatch(state.tr.delete(from, from + 1).scrollIntoView())
            }
            return true
        }

export const deleteNameBackward = deleteName(-1)
export const deleteNameForward = deleteName(1)

const isGreeting = (block: Node | null): boolean => {
    let hasName = false
    block?.forEach((child) => {
        hasName ||= child.type === name
    })
    return hasName
}

export const getBodyStart = (doc: Node): Selection => {
    const first = doc.firstChild
    if (first && doc.childCount > 1 && isGreeting(first)) {
        return Selection.near(doc.resolve(first.nodeSize + 1))
    }
    return Selection.atStart(doc)
}
