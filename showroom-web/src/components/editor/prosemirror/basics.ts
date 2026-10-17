import type { Node } from "prosemirror-model"
import { Plugin } from "prosemirror-state"
import { Decoration, DecorationSet } from "prosemirror-view"
import { schema } from "./schema"

const { paragraph, heading, caption, attribution } = schema.nodes

export const placeholders = (texts: { body: string, caption: string, attribution: string }) => new Plugin({
    props: {
        decorations: (state) => {
            const decorations: Array<Decoration> = []
            const { doc } = state

            const first = doc.firstChild
            if (doc.childCount === 1 && first?.type === paragraph && first.content.size === 0) {
                decorations.push(Decoration.node(0, first.nodeSize, { class: "is-empty", "data-placeholder": texts.body }))
            }

            doc.descendants((node, pos) => {
                if (node.isTextblock) {
                    if (node.content.size === 0 && (node.type === caption || node.type === attribution)) {
                        const text = node.type === caption ? texts.caption : texts.attribution
                        decorations.push(Decoration.node(pos, pos + node.nodeSize, { class: "is-empty", "data-placeholder": text }))
                    }
                    return false
                }
                return true
            })

            return DecorationSet.create(doc, decorations)
        },
    },
})

export const needsTrailingParagraph = (doc: Node): boolean => {
    const last = doc.lastChild
    return !last || (last.type !== paragraph && last.type !== heading)
}

export const ensureTrailingParagraph = (doc: Node): Node =>
    needsTrailingParagraph(doc) ? doc.copy(doc.content.addToEnd(paragraph.create())) : doc

export const trailingParagraph = () => new Plugin({
    appendTransaction: (transactions, _previous, state) => {
        if (!transactions.some((transaction) => transaction.docChanged) || !needsTrailingParagraph(state.doc)) {
            return null
        }
        return state.tr.insert(state.doc.content.size, paragraph.create())
    },
})
