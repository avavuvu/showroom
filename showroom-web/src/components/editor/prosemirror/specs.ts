import type { MarkSpec, NodeSpec } from "prosemirror-model"
import { isAllowedHref } from "./url"

export const paragraphSpec: NodeSpec = {
    content: "inline*",
    group: "block quotable",
    parseDOM: [{ tag: "p" }],
    toDOM: () => ["p", 0],
}

export const blockquoteSpec: NodeSpec = {
    content: "block+",
    group: "block",
    defining: true,
    parseDOM: [{ tag: "blockquote", context: "pullquote/", skip: true, priority: 60 }, { tag: "blockquote" }],
    toDOM: () => ["blockquote", 0],
}

export const bulletListSpec: NodeSpec = {
    content: "listItem+",
    group: "block quotable",
    parseDOM: [{ tag: "ul" }],
    toDOM: () => ["ul", 0],
}

export const orderedListSpec: NodeSpec = {
    attrs: { start: { default: 1, validate: "number" } },
    content: "listItem+",
    group: "block quotable",
    parseDOM: [
        {
            tag: "ol",
            getAttrs: (element) => ({ start: Math.max(0, Number(element.getAttribute("start") ?? 1) || 1) }),
        },
    ],
    toDOM: (node) => ["ol", node.attrs.start === 1 ? {} : { start: node.attrs.start }, 0],
}

export const listItemSpec: NodeSpec = {
    content: "paragraph block*",
    defining: true,
    parseDOM: [{ tag: "li" }],
    toDOM: () => ["li", 0],
}

export const textSpec: NodeSpec = { group: "inline" }

export const hardBreakSpec: NodeSpec = {
    inline: true,
    group: "inline",
    selectable: false,
    parseDOM: [{ tag: "br" }],
    toDOM: () => ["br"],
}

export const linkSpec: MarkSpec = {
    attrs: { href: { validate: "string" } },
    inclusive: false,
    parseDOM: [
        {
            tag: "a[href]",
            getAttrs: (element) => {
                const href = element.getAttribute("href")?.trim() ?? ""
                return isAllowedHref(href) ? { href } : false
            },
        },
    ],
    toDOM: (mark) => ["a", { href: mark.attrs.href, rel: "noopener noreferrer nofollow" }, 0],
}

export const boldSpec: MarkSpec = {
    parseDOM: [
        { tag: "strong" },
        { tag: "b", getAttrs: (element) => element.style.fontWeight !== "normal" && null },
        { style: "font-weight=400", clearMark: (mark) => mark.type.name === "bold" },
        { style: "font-weight", getAttrs: (value) => /^(bold(er)?|[5-9]\d{2,})$/.test(value) && null },
    ],
    toDOM: () => ["strong", 0],
}

export const italicSpec: MarkSpec = {
    parseDOM: [
        { tag: "i" },
        { tag: "em" },
        { style: "font-style=italic" },
        { style: "font-style=normal", clearMark: (mark) => mark.type.name === "italic" },
    ],
    toDOM: () => ["em", 0],
}

export const underlineSpec: MarkSpec = {
    parseDOM: [{ tag: "u" }, { style: "text-decoration=underline", consuming: false }],
    toDOM: () => ["u", 0],
}

export const strikeSpec: MarkSpec = {
    parseDOM: [{ tag: "s" }, { tag: "del" }, { tag: "strike" }, { style: "text-decoration=line-through", consuming: false }],
    toDOM: () => ["s", 0],
}

export const codeSpec: MarkSpec = {
    parseDOM: [{ tag: "code" }],
    toDOM: () => ["code", 0],
}
