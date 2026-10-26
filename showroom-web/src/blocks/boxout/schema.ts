import type { NodeSpec } from "prosemirror-model"
import type { BoxoutStyle } from "../../../bindings/types/BoxoutStyle"
import type { BoxoutWidth } from "../../../bindings/types/BoxoutWidth"

const STYLES: ReadonlyArray<BoxoutStyle> = ["bordered", "brand", "trim"]
const WIDTHS: ReadonlyArray<BoxoutWidth> = ["wide", "text", "left", "right"]

export const readBoxoutStyle = (value: unknown): BoxoutStyle =>
    STYLES.find((style) => style === value) ?? "bordered"

export const readBoxoutWidth = (value: unknown): BoxoutWidth =>
    WIDTHS.find((width) => width === value) ?? "wide"

export const boxoutSpec: NodeSpec = {
    attrs: {
        style: { default: "bordered" },
        width: { default: "wide" },
    },
    content: "block+",
    defining: true,
    parseDOM: [
        {
            tag: "aside.boxout",
            priority: 60,
            getAttrs: (element) => ({
                style: readBoxoutStyle(element.dataset.style),
                width: readBoxoutWidth(element.dataset.width),
            }),
            contentElement: (element) => element.querySelector<HTMLElement>(".boxout-content") ?? element,
        },
    ],
    toDOM: (node) => ["aside", { class: "boxout", "data-style": node.attrs.style, "data-width": node.attrs.width }, 0],
}
