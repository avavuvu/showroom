import type { Mark, ResolvedPos } from "prosemirror-model"
import { type Command, type EditorState, Plugin, PluginKey, TextSelection } from "prosemirror-state"
import { Decoration, DecorationSet, type EditorView } from "prosemirror-view"
import { schema } from "../prosemirror/schema"
import { displayHref, normalizeHref } from "../prosemirror/url"

export interface Editing {
    from: number
    to: number
    href: string
}

interface LinkState {
    editing: Editing | null
}

type LinkMeta = { open: Editing } | { shouldClose: true }

export interface LinkRange {
    from: number
    to: number
    mark: Mark
}

const linkKey = new PluginKey<LinkState>("link")
const link = schema.marks.link

const findLinkRange = ($pos: ResolvedPos): LinkRange | null => {
    const parent = $pos.parent
    let child = parent.childAfter($pos.parentOffset)
    if (!child.node || !link.isInSet(child.node.marks)) {
        if ($pos.parentOffset === 0) {
            return null
        }
        child = parent.childBefore($pos.parentOffset)
    }
    const mark = child.node && link.isInSet(child.node.marks)
    if (!child.node || !mark) {
        return null
    }

    let startIndex = child.index
    let from = $pos.start() + child.offset
    let endIndex = startIndex + 1
    let to = from + child.node.nodeSize

    while (startIndex > 0 && mark.isInSet(parent.child(startIndex - 1).marks)) {
        startIndex--
        from -= parent.child(startIndex).nodeSize
    }
    while (endIndex < parent.childCount && mark.isInSet(parent.child(endIndex).marks)) {
        to += parent.child(endIndex).nodeSize
        endIndex++
    }
    return { from, to, mark }
}

export const getSelectedLink = (state: EditorState): LinkRange | null => {
    const { selection } = state
    if (!(selection instanceof TextSelection)) {
        return null
    }
    const range = findLinkRange(selection.$from)
    if (!range) {
        return null
    }
    if (!selection.empty && (selection.from < range.from || selection.to > range.to)) {
        return null
    }
    return range
}

export const canLink = (state: EditorState): boolean => {
    const { selection } = state
    return (
        selection instanceof TextSelection &&
        selection.$from.parent.type.allowsMarkType(link) &&
        selection.$to.parent.type.allowsMarkType(link)
    )
}

export const getLinkEditing = (state: EditorState): Editing | null => linkKey.getState(state)?.editing ?? null

export const openLink: Command = (state, dispatch) => {
    if (!canLink(state)) {
        return false
    }
    if (dispatch && !getLinkEditing(state)) {
        const existing = getSelectedLink(state)
        const editing: Editing = existing
            ? { from: existing.from, to: existing.to, href: existing.mark.attrs.href }
            : { from: state.selection.from, to: state.selection.to, href: "" }
        dispatch(state.tr.setMeta(linkKey, { open: editing } satisfies LinkMeta))
    }
    return true
}

export const closeLink = (view: EditorView) => {
    if (!view.isDestroyed && getLinkEditing(view.state)) {
        view.dispatch(view.state.tr.setMeta(linkKey, { shouldClose: true } satisfies LinkMeta))
    }
}

export const getLinkText = (state: EditorState, range: { from: number, to: number }): string =>
    state.doc.textBetween(range.from, range.to, " ")

export const applyLink = (view: EditorView, rawHref: string, rawText: string): string | null => {
    const { state } = view
    const editing = getLinkEditing(state)
    if (!editing) {
        return null
    }

    const href = rawHref.trim() ? normalizeHref(rawHref) : null
    if (rawHref.trim() && !href) {
        return "Use a web address, an email address or a phone number for the link."
    }

    const current = getLinkText(state, editing)
    const label = rawText.trim() ? rawText : current || (href ? displayHref(href) : "")
    const transaction = state.tr
    let end = editing.to

    if (label !== current) {
        if (!label) {
            return null
        }
        const marks = (state.doc.nodeAt(editing.from)?.marks ?? state.selection.$from.marks()).filter((mark) => mark.type !== link)
        transaction.replaceWith(editing.from, editing.to, schema.text(label, marks))
        end = editing.from + label.length
    }

    transaction.removeMark(editing.from, end, link)
    if (href) {
        transaction.addMark(editing.from, end, link.create({ href }))
    }
    transaction.setSelection(TextSelection.create(transaction.doc, end))

    view.dispatch(transaction.setMeta(linkKey, { shouldClose: true } satisfies LinkMeta).scrollIntoView())
    return null
}

export const removeLink = (view: EditorView) => {
    const { state } = view
    const editing = getLinkEditing(state)
    const range = editing?.href ? editing : getSelectedLink(state)
    if (!range) {
        return
    }
    view.dispatch(state.tr.removeMark(range.from, range.to, link).setMeta(linkKey, { shouldClose: true } satisfies LinkMeta))
}

export const linkPlugin = () => new Plugin<LinkState>({
    key: linkKey,
    state: {
        init: () => ({ editing: null }),
        apply: (transaction, value) => {
            const meta = transaction.getMeta(linkKey) as LinkMeta | undefined
            if (meta && "open" in meta) {
                return { editing: meta.open }
            }
            if (meta && "shouldClose" in meta) {
                return { editing: null }
            }
            if (value.editing && transaction.docChanged) {
                const from = transaction.mapping.map(value.editing.from, 1)
                const to = Math.max(from, transaction.mapping.map(value.editing.to, -1))
                return { editing: { ...value.editing, from, to } }
            }
            return value
        },
    },
    props: {
        decorations: (state) => {
            const editing = getLinkEditing(state)
            if (!editing || editing.from === editing.to) {
                return null
            }
            return DecorationSet.create(state.doc, [Decoration.inline(editing.from, editing.to, { class: "link-target" })])
        },
        handlePaste: (view, event) => {
            const { selection } = view.state
            const text = event.clipboardData?.getData("text/plain")?.trim() ?? ""
            if (selection.empty || !canLink(view.state)) {
                return false
            }
            if (!/^(https?:\/\/|mailto:)\S+$/i.test(text)) {
                return false
            }

            const href = normalizeHref(text)
            if (!href) {
                return false
            }
            view.dispatch(view.state.tr.addMark(selection.from, selection.to, link.create({ href })))
            return true
        },
    },
})
