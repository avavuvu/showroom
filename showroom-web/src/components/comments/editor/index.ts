import type { CommentEditor } from "@setups"
import { baseKeymap, toggleMark } from "prosemirror-commands"
import { history, redo, undo } from "prosemirror-history"
import { inputRules, undoInputRule, wrappingInputRule } from "prosemirror-inputrules"
import { keymap } from "prosemirror-keymap"
import type { Node } from "prosemirror-model"
import { liftListItem, sinkListItem, splitListItem } from "prosemirror-schema-list"
import { type Command, EditorState, Plugin } from "prosemirror-state"
import { Decoration, DecorationSet, EditorView } from "prosemirror-view"
import { findLinkHref } from "../../editor/prosemirror/url"
import { commentSchema } from "./schema"

const { paragraph, blockquote, bulletList, orderedList, listItem, hardBreak } = commentSchema.nodes
const { bold, italic, code, link } = commentSchema.marks

const readDoc = (content: string): Node | null => {
    if (!content) {
        return null
    }
    try {
        const doc = commentSchema.nodeFromJSON(JSON.parse(content))
        doc.check()
        return doc
    } catch {
        return null
    }
}

const createParagraph = (text: string): Node => {
    const content = text.split("\n").flatMap((line, index) => [
        ...(index > 0 ? [hardBreak.create()] : []),
        ...(line ? [commentSchema.text(line)] : []),
    ])
    return paragraph.create(null, content)
}

const createDocFromText = (text: string): Node => {
    const trimmed = text.trim()
    const blocks = trimmed ? trimmed.split(/\n\s*\n/).map(createParagraph) : [paragraph.create()]
    return commentSchema.node("doc", null, blocks)
}

const isEmptyDoc = (doc: Node): boolean => {
    const first = doc.firstChild
    return doc.childCount === 1 && first?.type === paragraph && first.content.size === 0
}

const placeholder = (text: string) => new Plugin({
    props: {
        decorations: ({ doc }) => {
            const first = doc.firstChild
            if (!first || !isEmptyDoc(doc)) {
                return null
            }
            return DecorationSet.create(doc, [Decoration.node(0, first.nodeSize, { class: "is-empty", "data-placeholder": text })])
        },
    },
})

const pastedLinks = () => new Plugin({
    props: {
        handlePaste: (view, event) => {
            const text = event.clipboardData?.getData("text/plain")?.trim() ?? ""
            const href = findLinkHref(text)
            const { state } = view
            const { selection } = state
            if (!href || selection.$from.parent.type.spec.code) {
                return false
            }
            if (selection.empty) {
                view.dispatch(state.tr.replaceSelectionWith(commentSchema.text(text, [link.create({ href })]), false).scrollIntoView())
            } else {
                view.dispatch(state.tr.addMark(selection.from, selection.to, link.create({ href })))
            }
            return true
        },
    },
})

const insertHardBreak: Command = (state, dispatch) => {
    if (dispatch) {
        dispatch(state.tr.replaceSelectionWith(hardBreak.create()).scrollIntoView())
    }
    return true
}

const buildRules = () => inputRules({
    rules: [
        wrappingInputRule(/^\s*>\s$/, blockquote),
        wrappingInputRule(/^\s*([-+*])\s$/, bulletList),
        wrappingInputRule(
            /^(\d+)\.\s$/,
            orderedList,
            (match) => ({ start: Number(match[1]) }),
            (match, node) => node.childCount + node.attrs.start === Number(match[1]),
        ),
    ],
})

export const commentEditor: CommentEditor = (_root, refs) => {
    const { fallback, content, surface } = refs

    const submit: Command = () => {
        content.form?.requestSubmit()
        return true
    }

    const sync = (doc: Node) => {
        content.value = isEmptyDoc(doc) ? "" : JSON.stringify(doc.toJSON())
    }

    const view: EditorView = new EditorView(surface, {
        state: EditorState.create({
            doc: readDoc(content.value) ?? createDocFromText(fallback.value),
            plugins: [
                buildRules(),
                keymap({
                    "Mod-z": undo,
                    "Shift-Mod-z": redo,
                    "Mod-y": redo,
                    "Mod-b": toggleMark(bold),
                    "Mod-i": toggleMark(italic),
                    "Mod-e": toggleMark(code),
                    "Mod-Enter": submit,
                    "Shift-Enter": insertHardBreak,
                    "Enter": splitListItem(listItem),
                    "Tab": sinkListItem(listItem),
                    "Shift-Tab": liftListItem(listItem),
                    "Backspace": undoInputRule,
                }),
                keymap(baseKeymap),
                history(),
                placeholder(fallback.placeholder),
                pastedLinks(),
            ],
        }),
        attributes: {
            class: "comment-editor-body comment-rich",
            role: "textbox",
            "aria-multiline": "true",
            "aria-label": "Comment",
            spellcheck: "true",
            autocapitalize: "sentences",
        },
        dispatchTransaction: (transaction) => {
            const state = view.state.apply(transaction)
            view.updateState(state)
            if (transaction.docChanged) {
                sync(state.doc)
            }
        },
    })

    sync(view.state.doc)
    fallback.disabled = true
    fallback.hidden = true
    content.disabled = false
    surface.hidden = false

    return {
        disconnect: () => view.destroy(),
    }
}
