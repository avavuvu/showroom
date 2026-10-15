import { Schema } from "prosemirror-model";
import { isAllowedHref, isAllowedImageSrc } from "./url";

const languageOf = (element: HTMLElement): string | null => {
    const className = element.querySelector("code")?.className ?? element.className;
    const language = /language-([\w+#.-]{1,32})/.exec(className)?.[1];
    return language ?? null;
};

const isQuoteFigure = (element: HTMLElement) =>
    element.classList.contains("pullquote") || element.classList.contains("quote") || !!element.querySelector("blockquote");

export const schema = new Schema({
    nodes: {
        doc: { content: "block+" },
        paragraph: {
            content: "inline*",
            group: "block quotable",
            parseDOM: [{ tag: "p" }],
            toDOM: () => ["p", 0],
        },
        heading: {
            attrs: { level: { default: 2, validate: "number" } },
            content: "inline*",
            group: "block quotable",
            defining: true,
            parseDOM: [
                { tag: "h1", attrs: { level: 2 } },
                { tag: "h2", attrs: { level: 2 } },
                { tag: "h3", attrs: { level: 3 } },
                { tag: "h4", attrs: { level: 4 } },
                { tag: "h5", attrs: { level: 4 } },
                { tag: "h6", attrs: { level: 4 } },
            ],
            toDOM: (node) => [`h${Math.min(4, Math.max(2, node.attrs.level))}`, 0],
        },
        blockquote: {
            content: "block+",
            group: "block",
            defining: true,
            parseDOM: [{ tag: "blockquote", context: "pullquote/", skip: true, priority: 60 }, { tag: "blockquote" }],
            toDOM: () => ["blockquote", 0],
        },
        pullquote: {
            content: "quotable+ attribution",
            group: "block",
            defining: true,
            isolating: true,
            parseDOM: [
                { tag: "figure", priority: 60, getAttrs: (element) => (isQuoteFigure(element) ? null : false) },
                { tag: "blockquote.pullquote", priority: 60 },
            ],
            toDOM: () => ["figure", { class: "pullquote" }, 0],
        },
        attribution: {
            content: "inline*",
            defining: true,
            parseDOM: [{ tag: "figcaption", context: "pullquote/" }],
            toDOM: () => ["figcaption", 0],
        },
        bulletList: {
            content: "listItem+",
            group: "block quotable",
            parseDOM: [{ tag: "ul" }],
            toDOM: () => ["ul", 0],
        },
        orderedList: {
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
        },
        listItem: {
            content: "paragraph block*",
            defining: true,
            parseDOM: [{ tag: "li" }],
            toDOM: () => ["li", 0],
        },
        codeBlock: {
            attrs: { language: { default: null } },
            content: "text*",
            marks: "",
            group: "block quotable",
            code: true,
            defining: true,
            parseDOM: [{ tag: "pre", preserveWhitespace: "full", getAttrs: (element) => ({ language: languageOf(element) }) }],
            toDOM: (node) => ["pre", ["code", node.attrs.language ? { class: `language-${node.attrs.language}` } : {}, 0]],
        },
        horizontalRule: {
            group: "block quotable",
            parseDOM: [{ tag: "hr" }],
            toDOM: () => ["hr"],
        },
        figure: {
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
                        if (isQuoteFigure(element) || !element.querySelector("img")) return false;
                        return { width: element.getAttribute("data-width") === "full" ? "full" : "normal" };
                    },
                },
            ],
            toDOM: (node) => ["figure", { class: "image-block", "data-width": node.attrs.width }, 0],
        },
        image: {
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
                        const src = element.getAttribute("src") ?? "";
                        return isAllowedImageSrc(src) ? { src, title: element.getAttribute("title") || null } : false;
                    },
                },
            ],
            toDOM: (node) => ["img", { src: node.attrs.src, alt: "", title: node.attrs.title }],
        },
        caption: {
            content: "inline*",
            defining: true,
            parseDOM: [{ tag: "figcaption", context: "figure/" }],
            toDOM: () => ["figcaption", 0],
        },
        text: { group: "inline" },
        hardBreak: {
            inline: true,
            group: "inline",
            selectable: false,
            parseDOM: [{ tag: "br" }],
            toDOM: () => ["br"],
        },
    },
    marks: {
        link: {
            attrs: { href: { validate: "string" } },
            inclusive: false,
            parseDOM: [
                {
                    tag: "a[href]",
                    getAttrs: (element) => {
                        const href = element.getAttribute("href")?.trim() ?? "";
                        return isAllowedHref(href) ? { href } : false;
                    },
                },
            ],
            toDOM: (mark) => ["a", { href: mark.attrs.href, rel: "noopener noreferrer nofollow" }, 0],
        },
        bold: {
            parseDOM: [
                { tag: "strong" },
                { tag: "b", getAttrs: (element) => element.style.fontWeight !== "normal" && null },
                { style: "font-weight=400", clearMark: (mark) => mark.type.name === "bold" },
                { style: "font-weight", getAttrs: (value) => /^(bold(er)?|[5-9]\d{2,})$/.test(value) && null },
            ],
            toDOM: () => ["strong", 0],
        },
        italic: {
            parseDOM: [
                { tag: "i" },
                { tag: "em" },
                { style: "font-style=italic" },
                { style: "font-style=normal", clearMark: (mark) => mark.type.name === "italic" },
            ],
            toDOM: () => ["em", 0],
        },
        underline: {
            parseDOM: [{ tag: "u" }, { style: "text-decoration=underline", consuming: false }],
            toDOM: () => ["u", 0],
        },
        strike: {
            parseDOM: [{ tag: "s" }, { tag: "del" }, { tag: "strike" }, { style: "text-decoration=line-through", consuming: false }],
            toDOM: () => ["s", 0],
        },
        code: {
            parseDOM: [{ tag: "code" }],
            toDOM: () => ["code", 0],
        },
    },
});
