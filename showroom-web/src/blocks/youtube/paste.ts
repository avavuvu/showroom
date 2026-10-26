import { closeHistory } from "prosemirror-history"
import { Plugin, TextSelection } from "prosemirror-state"
import type { EditorView } from "prosemirror-view"
import { schema } from "../../components/editor/prosemirror/schema"
import { normalizeHref } from "../../components/editor/prosemirror/url"
import { isEmptyTopLevelParagraph } from "../selection"
import { type YoutubeLink, parseYoutubeUrl } from "./url"

const pasteLink = (view: EditorView, text: string, href: string) => {
    const link = schema.text(text, [schema.marks.link.create({ href })])
    view.dispatch(closeHistory(view.state.tr.replaceSelectionWith(link, false)))
}

const replaceWithVideo = (view: EditorView, link: YoutubeLink) => {
    const { state } = view
    const { $from } = state.selection
    const start = $from.before(1)
    const end = $from.after(1)
    const video = schema.nodes.youtube.create({ id: link.id, start: link.start, short: link.isShort })
    const hasParagraphAfter = state.doc.resolve(end).nodeAfter?.type === schema.nodes.paragraph
    const transaction = state.tr.replaceWith(start, end, hasParagraphAfter ? [video] : [video, schema.nodes.paragraph.create()])
    transaction.setSelection(TextSelection.create(transaction.doc, start + video.nodeSize + 1))
    view.dispatch(closeHistory(transaction).scrollIntoView())
}

export const youtubePlugin = () => new Plugin({
    props: {
        handlePaste: (view, event) => {
            const text = event.clipboardData?.getData("text/plain")?.trim() ?? ""
            const href = text && !/\s/.test(text) ? normalizeHref(text) : null
            const link = href ? parseYoutubeUrl(href) : null
            if (!href || !link || !isEmptyTopLevelParagraph(view.state)) {
                return false
            }
            pasteLink(view, text, href)
            replaceWithVideo(view, link)
            return true
        },
    },
})
