import type { NodeSpec } from "prosemirror-model";

const musicAttrs = (element: HTMLElement) => {
    try {
        const attrs = JSON.parse(element.getAttribute("data-music") ?? "");
        return attrs && typeof attrs === "object" && attrs.links && typeof attrs.links === "object" ? attrs : false;
    } catch {
        return false;
    }
};

export const musicSpec: NodeSpec = {
    attrs: {
        kind: { default: "track" },
        title: { default: "" },
        artist: { default: "" },
        artwork: { default: null },
        source: { default: null },
        links: { default: {} },
    },
    atom: true,
    selectable: true,
    draggable: true,
    parseDOM: [{ tag: "figure.music-card[data-music]", priority: 70, getAttrs: musicAttrs }],
    toDOM: (node) => ["figure", { class: "music-card", "data-music": JSON.stringify(node.attrs) }],
};
