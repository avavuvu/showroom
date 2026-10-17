import type { NodeSpec } from "prosemirror-model"
import { isAllowedImageSrc } from "../../components/editor/prosemirror/url"

export const isQuoteFigure = (element: HTMLElement) =>
    element.classList.contains("pullquote") || element.classList.contains("quote") || !!element.querySelector("blockquote")

export const figureSpec: NodeSpec = {
    attrs: { width: { default: "normal", validate: "string" } },
    content: "image caption",
    group: "block quotable",
    defining: true,
    isolating: true,
    draggable: true,
    parseDOM: [
        {
            tag: "figure",
            getAttrs: (element) => {
                if (isQuoteFigure(element) || !element.querySelector("img")) {
                    return false
                }
                return { width: element.getAttribute("data-width") === "full" ? "full" : "normal" }
            },
        },
    ],
    toDOM: (node) => ["figure", { class: "image-block", "data-width": node.attrs.width }, 0],
}

export const imageSpec: NodeSpec = {
    attrs: {
        src: { default: "", validate: "string" },
        title: { default: null },
        publicId: { default: null },
    },
    selectable: false,
    parseDOM: [
        {
            tag: "img[src]",
            getAttrs: (element) => {
                const src = element.getAttribute("src") ?? ""
                return isAllowedImageSrc(src) ? { src, title: element.getAttribute("title") || null } : false
            },
        },
    ],
    toDOM: (node) => ["img", { src: node.attrs.src, alt: "", title: node.attrs.title }],
}

export const captionSpec: NodeSpec = {
    content: "inline*",
    defining: true,
    parseDOM: [{ tag: "figcaption", context: "figure/" }],
    toDOM: () => ["figcaption", 0],
}
