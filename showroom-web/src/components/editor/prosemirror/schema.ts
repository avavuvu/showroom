import { Schema } from "prosemirror-model"
import { captionSpec, figureSpec, imageSpec, isQuoteFigure } from "../../../blocks/image/schema"
import { musicSpec } from "../../../blocks/music/schema"
import { youtubeSpec } from "../../../blocks/youtube/schema"
import { boxoutSpec } from "../../../blocks/boxout/schema"
import {
    blockquoteSpec,
    boldSpec,
    bulletListSpec,
    codeSpec,
    hardBreakSpec,
    italicSpec,
    linkSpec,
    listItemSpec,
    orderedListSpec,
    paragraphSpec,
    strikeSpec,
    textSpec,
    underlineSpec,
} from "./specs"

const getLanguage = (element: HTMLElement): string | null => {
    const className = element.querySelector("code")?.className ?? element.className
    const language = /language-([\w+#.-]{1,32})/.exec(className)?.[1]
    return language ?? null
}

export const NAME_TEXT = "{{name}}"

export const schema = new Schema({
    nodes: {
        doc: { content: "(block | music | youtube | boxout)+" },
        paragraph: paragraphSpec,
        heading: {
            attrs: { level: { default: 2, validate: "number" }, id: { default: null, validate: "string|null" } },
            content: "inline*",
            group: "block quotable",
            defining: true,
            parseDOM: [
                { tag: "h1", getAttrs: (element) => ({ level: 2, id: element.dataset.sectionId ?? null }) },
                { tag: "h2", getAttrs: (element) => ({ level: 2, id: element.dataset.sectionId ?? null }) },
                { tag: "h3", getAttrs: (element) => ({ level: 3, id: element.dataset.sectionId ?? null }) },
                { tag: "h4", getAttrs: (element) => ({ level: 4, id: element.dataset.sectionId ?? null }) },
                { tag: "h5", getAttrs: (element) => ({ level: 4, id: element.dataset.sectionId ?? null }) },
                { tag: "h6", getAttrs: (element) => ({ level: 4, id: element.dataset.sectionId ?? null }) },
            ],
            toDOM: (node) => [
                `h${Math.min(4, Math.max(2, node.attrs.level))}`,
                node.attrs.id ? { "data-section-id": node.attrs.id } : {},
                0,
            ],
        },
        blockquote: blockquoteSpec,
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
        bulletList: bulletListSpec,
        orderedList: orderedListSpec,
        listItem: listItemSpec,
        codeBlock: {
            attrs: { language: { default: null } },
            content: "text*",
            marks: "",
            group: "block quotable",
            code: true,
            defining: true,
            parseDOM: [{ tag: "pre", preserveWhitespace: "full", getAttrs: (element) => ({ language: getLanguage(element) }) }],
            toDOM: (node) => ["pre", ["code", node.attrs.language ? { class: `language-${node.attrs.language}` } : {}, 0]],
        },
        horizontalRule: {
            group: "block quotable",
            parseDOM: [{ tag: "hr" }],
            toDOM: () => ["hr"],
        },
        figure: figureSpec,
        image: imageSpec,
        caption: captionSpec,
        music: musicSpec,
        youtube: youtubeSpec,
        boxout: boxoutSpec,
        text: textSpec,
        name: {
            inline: true,
            group: "inline",
            atom: true,
            selectable: true,
            draggable: false,
            attrs: { fallback: { default: "" } },
            leafText: () => NAME_TEXT,
            parseDOM: [{ tag: "span[data-name]", getAttrs: (element) => ({ fallback: element.dataset.fallback ?? "" }) }],
            toDOM: (node) => {
                const fallback = node.attrs.fallback as string
                const attrs = { class: "name-token", "data-name": "", contenteditable: "false", title: "The subscriber's name" }
                if (!fallback) {
                    return ["span", attrs, "name"]
                }
                return ["span", { ...attrs, "data-fallback": fallback }, "name", ["span", { class: "name-token-fallback" }, fallback]]
            },
        },
        hardBreak: hardBreakSpec,
    },
    marks: {
        link: linkSpec,
        bold: boldSpec,
        italic: italicSpec,
        underline: underlineSpec,
        strike: strikeSpec,
        code: codeSpec,
    },
})
