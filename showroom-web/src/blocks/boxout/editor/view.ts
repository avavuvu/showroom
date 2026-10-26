import type { Node as ProseNode } from "prosemirror-model"
import type { EditorView, NodeView, ViewMutationRecord } from "prosemirror-view"
import { provide } from "../../../../bindings/boutique/setups"
import type { BoxoutAttrs } from "../../../../bindings/types/BoxoutAttrs"
import { cloneTemplate } from "../../decorations"
import { unwrapBoxoutAt } from "../commands"
import { boxoutEditorKey } from "./handle"

const CONTROLS = ".boxout-controls, .boxout-handle"

class BoxoutView implements NodeView {
    readonly dom: HTMLElement
    readonly contentDOM: HTMLElement
    private listeners = new Set<() => void>()

    constructor(
        private node: ProseNode,
        private view: EditorView,
        private getPos: () => number | undefined,
        template: HTMLTemplateElement,
    ) {
        this.dom = cloneTemplate(template)
        const content = this.dom.querySelector<HTMLElement>(".boxout-content")
        if (!content) {
            throw new Error("the boxout template has no content element")
        }
        this.contentDOM = content
        this.render()

        provide(this.dom, boxoutEditorKey, {
            getAttrs: () => this.attrs,
            change: (fields) => this.change(fields),
            unwrap: () => this.unwrap(),
            remove: () => this.remove(),
            getColumn: () => this.getColumn(),
            onUpdate: (listener, signal) => {
                this.listeners.add(listener)
                signal.addEventListener("abort", () => this.listeners.delete(listener), { once: true })
                listener()
            },
        })
    }

    private get attrs(): BoxoutAttrs {
        return this.node.attrs as BoxoutAttrs
    }

    private render() {
        this.dom.dataset.style = this.attrs.style
        this.dom.dataset.width = this.attrs.width
        for (const listener of this.listeners) {
            listener()
        }
    }

    private getColumn(): DOMRect {
        const column = this.dom.parentElement ?? this.dom
        const bounds = column.getBoundingClientRect()
        const style = getComputedStyle(column)
        const left = bounds.left + parseFloat(style.paddingLeft)
        const right = bounds.right - parseFloat(style.paddingRight)
        return new DOMRect(left, bounds.top, right - left, bounds.height)
    }

    private change(fields: Partial<BoxoutAttrs>) {
        const pos = this.getPos()
        if (pos == null || this.view.isDestroyed) {
            return
        }
        const attrs = { ...this.attrs, ...fields }
        if (attrs.style === this.attrs.style && attrs.width === this.attrs.width) {
            this.render()
            return
        }
        this.view.dispatch(this.view.state.tr.setNodeMarkup(pos, null, attrs))
    }

    private unwrap() {
        const pos = this.getPos()
        if (pos == null) {
            return
        }
        const transaction = this.view.state.tr
        if (unwrapBoxoutAt(transaction, pos)) {
            this.view.dispatch(transaction.scrollIntoView())
            this.view.focus()
        }
    }

    private remove() {
        const pos = this.getPos()
        if (pos == null) {
            return
        }
        this.view.dispatch(this.view.state.tr.delete(pos, pos + this.node.nodeSize))
        this.view.focus()
    }

    update(node: ProseNode): boolean {
        if (node.type !== this.node.type) {
            return false
        }
        this.node = node
        this.render()
        return true
    }

    selectNode() {
        this.dom.classList.add("is-selected")
    }

    deselectNode() {
        this.dom.classList.remove("is-selected")
    }

    stopEvent(event: Event): boolean {
        return event.target instanceof Element && event.target.closest(CONTROLS) !== null && this.dom.contains(event.target)
    }

    ignoreMutation(mutation: ViewMutationRecord): boolean {
        return !this.contentDOM.contains(mutation.target)
    }

    destroy() {
        this.listeners.clear()
    }
}

export const boxoutView = (template: HTMLTemplateElement) =>
    (node: ProseNode, view: EditorView, getPos: () => number | undefined) => new BoxoutView(node, view, getPos, template)
