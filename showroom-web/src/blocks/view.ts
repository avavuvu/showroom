import type { Node as ProseNode } from "prosemirror-model"
import type { Decoration, EditorView, NodeView } from "prosemirror-view"
import { type ContextKey, provide } from "../../bindings/boutique/setups"
import { cloneTemplate } from "./decorations"

const NOTICE_MS = 8_000

const views = new WeakMap<Node, BlockView<object, object>>()

const isField = (target: EventTarget | null): target is HTMLInputElement | HTMLTextAreaElement =>
    target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement

export interface BlockHandle<Attrs> {
    getAttrs(): Attrs
    set(attrs: Attrs): void
    change(fields: Partial<Attrs>): void
    remove(): void
    getStatus(): string
    isBusy(): boolean
    focusEditor(): void
    onUpdate(listener: () => void, signal: AbortSignal): void
}

export interface BlockViewOptions<Attrs extends object, Actions extends object> {
    template: HTMLTemplateElement
    key: ContextKey<BlockHandle<Attrs> & Actions>
    actions: (block: BlockView<Attrs, Actions>) => Actions
}

export const blockView = <Attrs extends object, Actions extends object>(options: BlockViewOptions<Attrs, Actions>) =>
    (node: ProseNode, view: EditorView, getPos: () => number | undefined, decorations: ReadonlyArray<Decoration>) =>
        new BlockView(node, view, getPos, decorations, options)

export const setFieldValue = (field: HTMLInputElement | HTMLTextAreaElement, value: string) => {
    if (document.activeElement !== field && field.value !== value) {
        field.value = value
    }
}

export const setImageSource = (image: HTMLImageElement, src: string | null | undefined) => {
    if (src) {
        if (image.getAttribute("src") !== src) {
            image.src = src
        }
        image.hidden = false
    } else {
        image.removeAttribute("src")
        image.hidden = true
    }
}

export class BlockView<Attrs extends object, Actions extends object> implements NodeView {
    readonly dom: HTMLElement
    private listeners = new Set<() => void>()
    private currentStatus = ""
    private isBusy = false
    private noticeTimer: ReturnType<typeof setTimeout> | undefined

    constructor(
        private node: ProseNode,
        readonly view: EditorView,
        private getNodePos: () => number | undefined,
        private outerDecorations: ReadonlyArray<Decoration>,
        options: BlockViewOptions<Attrs, Actions>,
    ) {
        this.dom = cloneTemplate(options.template)
        views.set(this.dom, this)

        this.dom.addEventListener("pointerdown", (event) => {
            if (isField(event.target)) {
                this.dom.draggable = false
            }
        })
        this.dom.addEventListener("focusout", (event) => {
            if (!isField(event.relatedTarget) || !this.dom.contains(event.relatedTarget)) {
                this.dom.draggable = this.node.type.spec.draggable ?? false
            }
        })

        const handle: BlockHandle<Attrs> = {
            getAttrs: () => this.attrs,
            set: (attrs) => this.set(attrs),
            change: (fields) => this.change(fields),
            remove: () => this.remove(),
            getStatus: () => this.currentStatus,
            isBusy: () => this.isBusy,
            focusEditor: () => this.view.focus(),
            onUpdate: (listener, signal) => {
                this.listeners.add(listener)
                signal.addEventListener("abort", () => this.listeners.delete(listener), { once: true })
                listener()
            },
        }
        provide(this.dom, options.key, { ...handle, ...options.actions(this) })
    }

    static findAt(view: EditorView, pos: number): BlockView<object, object> | undefined {
        const dom = view.nodeDOM(pos)
        return dom ? views.get(dom) : undefined
    }

    get attrs(): Attrs {
        return this.node.attrs as Attrs
    }

    getPos(): number | undefined {
        return this.getNodePos()
    }

    getDecorations(): ReadonlyArray<Decoration> {
        return this.outerDecorations
    }

    set(attrs: Attrs) {
        const pos = this.getNodePos()
        if (pos == null || this.view.isDestroyed) {
            return
        }
        this.view.dispatch(this.view.state.tr.setNodeMarkup(pos, null, attrs))
    }

    change(fields: Partial<Attrs>) {
        this.set({ ...this.attrs, ...fields })
    }

    remove() {
        const pos = this.getNodePos()
        if (pos == null) {
            return
        }
        this.view.dispatch(this.view.state.tr.delete(pos, pos + this.node.nodeSize))
        this.view.focus()
    }

    notify() {
        for (const listener of this.listeners) {
            listener()
        }
    }

    setStatus(text: string, isBusy = false) {
        clearTimeout(this.noticeTimer)
        this.currentStatus = text
        this.isBusy = isBusy
        this.notify()
    }

    showNotice(message: string) {
        this.setStatus(message)
        this.noticeTimer = setTimeout(() => this.setStatus(""), NOTICE_MS)
    }

    async run<T>(text: string, task: () => Promise<T>): Promise<T | undefined> {
        if (this.isBusy) {
            return undefined
        }
        this.setStatus(text, true)
        try {
            return await task()
        } finally {
            this.setStatus("")
        }
    }

    focusField() {
        this.dom.querySelector<HTMLInputElement | HTMLTextAreaElement>("input, textarea")?.focus()
    }

    update(node: ProseNode, decorations: ReadonlyArray<Decoration>): boolean {
        if (node.type !== this.node.type) {
            return false
        }
        this.node = node
        this.outerDecorations = decorations
        this.notify()
        return true
    }

    stopEvent(event: Event): boolean {
        return event.target instanceof Element && this.dom.contains(event.target) && event.target.closest("input, textarea, button, a") !== null
    }

    ignoreMutation(): boolean {
        return true
    }

    destroy() {
        clearTimeout(this.noticeTimer)
        this.listeners.clear()
    }
}
