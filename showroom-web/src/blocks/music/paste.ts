import { type EditorState, TextSelection } from "prosemirror-state"
import type { EditorView } from "prosemirror-view"
import { schema } from "../../components/editor/prosemirror/schema"
import { isAllowedHref } from "../../components/editor/prosemirror/url"
import { TrackedDecorations, cloneTemplate, createWidget } from "../decorations"
import { BlockView } from "../view"
import { type Resolved, createAttrs, getLookupProblem, hasDetails, looksLikeMusicLink, resolveMusic } from "./attrs"

const lookups = new TrackedDecorations("music-lookup")
const NOTICE_MS = 8_000

const createNoticeRenderer = (template: HTMLTemplateElement, message: string, isError: boolean) => () => {
    const element = cloneTemplate(template)
    element.classList.toggle("is-error", isError)
    const label = element.querySelector(".music-lookup-message")
    if (label) {
        label.textContent = message
        if (isError) {
            label.setAttribute("role", "alert")
        }
    }
    return element
}

const isEmptyTopLevelParagraph = (state: EditorState): boolean => {
    const { selection } = state
    const { $from } = selection
    return selection.empty && $from.depth === 1 && $from.parent.type === schema.nodes.paragraph && $from.parent.content.size === 0
}

const resolvePastedLink = async (view: EditorView, template: HTMLTemplateElement, text: string, id: object) => {
    let resolved: Resolved | null = null
    let failure = "That link could not be looked up."
    try {
        resolved = await resolveMusic(text)
    } catch (error) {
        if (error instanceof Error) {
            failure = error.message
        }
    }
    if (view.isDestroyed) {
        return
    }

    const at = lookups.find(view.state, id)?.from
    const transaction = lookups.change(view.state.tr, { remove: id })
    if (at == null) {
        view.dispatch(transaction)
        return
    }

    const existing = transaction.doc.nodeAt(at)
    const replacing = existing?.type === schema.nodes.paragraph && existing.content.size === 0 ? existing : null

    if (resolved) {
        const attrs = createAttrs(resolved)
        const music = schema.nodes.music.create(attrs)
        if (replacing) {
            transaction.replaceWith(at, at + replacing.nodeSize, music)
        } else {
            transaction.insert(at, music)
        }
        view.dispatch(transaction.scrollIntoView())

        const card = BlockView.findAt(view, at)
        const problem = getLookupProblem(resolved, null)
        if (problem) {
            card?.showNotice(problem)
        }
        if (!hasDetails(attrs)) {
            card?.focusField()
        }
        return
    }

    const marks = isAllowedHref(text) ? [schema.marks.link.create({ href: text })] : []
    const content = schema.text(text, marks)
    if (replacing) {
        transaction.insert(at + 1, content)
        transaction.setSelection(TextSelection.create(transaction.doc, at + 1 + text.length))
    } else {
        transaction.insert(at, schema.nodes.paragraph.create(null, content))
    }

    const noticeId = {}
    const message = `No music card was made. ${failure} The link was pasted as text instead.`
    lookups.change(transaction, { remove: id, add: createWidget(noticeId, at, createNoticeRenderer(template, message, true)) })
    view.dispatch(transaction.scrollIntoView())
    setTimeout(() => {
        if (!view.isDestroyed) {
            view.dispatch(lookups.change(view.state.tr, { remove: noticeId }))
        }
    }, NOTICE_MS)
}

export const musicPlugin = (template: HTMLTemplateElement) => lookups.plugin({
    handlePaste: (view, event) => {
        const text = event.clipboardData?.getData("text/plain")?.trim() ?? ""
        if (!text || /\s/.test(text) || !looksLikeMusicLink(text) || !isEmptyTopLevelParagraph(view.state)) {
            return false
        }

        const id = {}
        const pos = view.state.selection.$from.before(1)
        view.dispatch(lookups.change(view.state.tr, { add: createWidget(id, pos, createNoticeRenderer(template, "Looking up the link\u2026", false)) }))
        void resolvePastedLink(view, template, text, id)
        return true
    },
})
