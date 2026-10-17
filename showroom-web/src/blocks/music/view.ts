import "./index";

import type { Node } from "prosemirror-model";
import type { EditorView, NodeView } from "prosemirror-view";
import { provide } from "../../../bindings/boutique/setups";
import type { PickFiles } from "../../components/editor/images/file-picker";
import type { Uploads } from "../../components/editor/images/upload";
import { type MusicAttrs, lookupProblem, resolveMusic, withLink } from "./attrs";
import { musicNode } from "./handle";

export interface MusicViewOptions {
    template: HTMLTemplateElement;
    pickFiles: PickFiles;
    uploads: Uploads;
}

const NOTICE_MS = 8_000;

const isField = (target: EventTarget | null): target is HTMLInputElement => target instanceof HTMLInputElement;

const views = new WeakMap<globalThis.Node, MusicView>();

export class MusicView implements NodeView {
    dom: HTMLElement;
    private listeners = new Set<() => void>();
    private status = "";
    private busy = false;
    private noticeTimer: ReturnType<typeof setTimeout> | undefined;

    constructor(
        private node: Node,
        private view: EditorView,
        private getPos: () => number | undefined,
        private options: MusicViewOptions,
    ) {
        const cloned = options.template.content.firstElementChild?.cloneNode(true);
        this.dom = cloned instanceof HTMLElement ? cloned : document.createElement("figure");
        views.set(this.dom, this);

        this.dom.addEventListener("pointerdown", (event) => {
            if (isField(event.target)) this.dom.draggable = false;
        });
        this.dom.addEventListener("focusout", (event) => {
            if (!isField(event.relatedTarget) || !this.dom.contains(event.relatedTarget)) this.dom.draggable = true;
        });

        provide(this.dom, musicNode, {
            attrs: () => this.node.attrs as MusicAttrs,
            status: () => this.status,
            busy: () => this.busy,
            set: (attrs) => this.set(attrs),
            remove: () => this.remove(),
            addLink: (text) => this.addLink(text),
            pickCover: () => this.pickCover(),
            focusEditor: () => this.view.focus(),
            onUpdate: (listener, signal) => {
                this.listeners.add(listener);
                signal.addEventListener("abort", () => this.listeners.delete(listener), { once: true });
                listener();
            },
        });
    }

    static at(view: EditorView, pos: number): MusicView | undefined {
        const dom = view.nodeDOM(pos);
        return dom ? views.get(dom) : undefined;
    }

    announce(message: string) {
        this.notice(message);
    }

    focusTitle() {
        this.dom.querySelector<HTMLInputElement>(".music-title-input")?.focus();
    }

    update(node: Node): boolean {
        if (node.type !== this.node.type) return false;
        this.node = node;
        this.notify();
        return true;
    }

    stopEvent(event: Event): boolean {
        return event.target instanceof Element && this.dom.contains(event.target) && event.target.closest("input, button, a") !== null;
    }

    ignoreMutation(): boolean {
        return true;
    }

    destroy() {
        clearTimeout(this.noticeTimer);
        this.listeners.clear();
    }

    private notify() {
        for (const listener of this.listeners) listener();
    }

    private setStatus(status: string, busy: boolean) {
        clearTimeout(this.noticeTimer);
        this.status = status;
        this.busy = busy;
        this.notify();
    }

    private notice(message: string) {
        this.setStatus(message, false);
        this.noticeTimer = setTimeout(() => this.setStatus("", false), NOTICE_MS);
    }

    private set(attrs: MusicAttrs) {
        const pos = this.getPos();
        if (pos == null || this.view.isDestroyed) return;
        this.view.dispatch(this.view.state.tr.setNodeMarkup(pos, null, attrs));
    }

    private remove() {
        const pos = this.getPos();
        if (pos == null) return;
        this.view.dispatch(this.view.state.tr.delete(pos, pos + this.node.nodeSize));
        this.view.focus();
    }

    private pickCover() {
        this.options.pickFiles((files) => {
            if (files[0]) void this.uploadCover(files[0]);
        }, false);
    }

    private async uploadCover(file: File) {
        this.setStatus("Uploading cover\u2026", this.busy);
        const image = await this.options.uploads.upload(file);
        this.setStatus("", this.busy);
        if (image) this.set({ ...(this.node.attrs as MusicAttrs), artwork: image.src });
    }

    private async addLink(text: string): Promise<boolean> {
        const url = text.trim();
        if (!url || this.busy) return false;

        this.setStatus("Looking up\u2026", true);
        try {
            const resolved = await resolveMusic(url);
            const attrs = this.node.attrs as MusicAttrs;
            const problem = lookupProblem(resolved, attrs);
            this.setStatus("", false);
            this.set(withLink(attrs, resolved));
            if (problem) this.notice(problem);
            return true;
        } catch (error) {
            this.notice(error instanceof Error ? error.message : "That link could not be looked up.");
            return false;
        }
    }
}
