import type { NodeSpec } from "prosemirror-model"

const readString = (value: unknown, fallback: string | null) => (typeof value === "string" ? value : fallback)

const parseYoutubeAttrs = (element: HTMLElement) => {
    try {
        const attrs = JSON.parse(element.getAttribute("data-youtube") ?? "")
        if (!attrs || typeof attrs.id !== "string") {
            return false
        }
        return {
            id: attrs.id,
            start: typeof attrs.start === "number" ? attrs.start : null,
            short: attrs.short === true,
            hd: attrs.hd === true,
            title: readString(attrs.title, ""),
            channel: readString(attrs.channel, ""),
            channelUrl: readString(attrs.channelUrl, null),
            color: readString(attrs.color, null),
        }
    } catch {
        return false
    }
}

export const youtubeSpec: NodeSpec = {
    attrs: {
        id: { default: "" },
        start: { default: null },
        short: { default: false },
        hd: { default: false },
        title: { default: "" },
        channel: { default: "" },
        channelUrl: { default: null },
        color: { default: null },
    },
    atom: true,
    selectable: true,
    draggable: true,
    parseDOM: [{ tag: "figure.youtube-card[data-youtube]", priority: 70, getAttrs: parseYoutubeAttrs }],
    toDOM: (node) => ["figure", { class: "youtube-card", "data-youtube": JSON.stringify(node.attrs) }],
}
