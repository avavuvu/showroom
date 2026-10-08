import type { Node } from "prosemirror-model";
import { Plugin } from "prosemirror-state";
import { Decoration, DecorationSet } from "prosemirror-view";
import { schema } from "../schema";

const { paragraph, heading, caption, attribution } = schema.nodes;

export function placeholders(texts: { body: string; caption: string; attribution: string }) {
    return new Plugin({
        props: {
            decorations(state) {
                const decorations: Decoration[] = [];
                const { doc } = state;

                const first = doc.firstChild;
                if (doc.childCount === 1 && first?.type === paragraph && first.content.size === 0) {
                    decorations.push(Decoration.node(0, first.nodeSize, { class: "is-empty", "data-placeholder": texts.body }));
                }

                doc.descendants((node, pos) => {
                    if (node.isTextblock) {
                        if (node.content.size === 0 && (node.type === caption || node.type === attribution)) {
                            const text = node.type === caption ? texts.caption : texts.attribution;
                            decorations.push(Decoration.node(pos, pos + node.nodeSize, { class: "is-empty", "data-placeholder": text }));
                        }
                        return false;
                    }
                    return true;
                });

                return DecorationSet.create(doc, decorations);
            },
        },
    });
}

export function needsTrailingParagraph(doc: Node): boolean {
    const last = doc.lastChild;
    return !last || (last.type !== paragraph && last.type !== heading);
}

export function withTrailingParagraph(doc: Node): Node {
    return needsTrailingParagraph(doc) ? doc.copy(doc.content.addToEnd(paragraph.create())) : doc;
}

export function trailingParagraph() {
    return new Plugin({
        appendTransaction(transactions, _previous, state) {
            if (!transactions.some((tr) => tr.docChanged) || !needsTrailingParagraph(state.doc)) return null;
            return state.tr.insert(state.doc.content.size, paragraph.create());
        },
    });
}
