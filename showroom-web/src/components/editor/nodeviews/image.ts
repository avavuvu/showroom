import type { Node } from "prosemirror-model";
import type { EditorView, NodeView } from "prosemirror-view";
import type { Uploads } from "../plugins/upload";
import { schema } from "../schema";

export interface ImageViewOptions {
    template: HTMLTemplateElement;
    pickFiles: (callback: (files: File[]) => void, multiple: boolean) => void;
    uploads: Uploads;
}

export class ImageView implements NodeView {
    dom: HTMLElement;
    private img: HTMLImageElement;
    private actions: HTMLElement | null;
    private preview: string | null = null;

    constructor(
        private node: Node,
        private view: EditorView,
        private getPos: () => number | undefined,
        private options: ImageViewOptions,
    ) {
        const cloned = options.template.content.firstElementChild?.cloneNode(true);
        this.dom = cloned instanceof HTMLElement ? cloned : document.createElement("div");
        this.img = document.createElement("img");
        this.img.alt = "";
        this.img.draggable = false;
        this.dom.prepend(this.img);

        this.actions = this.dom.querySelector<HTMLElement>(".image-actions");
        this.actions?.addEventListener("click", (event) => {
            const action = (event.target as Element | null)?.closest<HTMLElement>("[data-action]")?.dataset.action;
            if (action) this.act(action);
        });

        this.render();
    }

    update(node: Node): boolean {
        if (node.type !== this.node.type) return false;
        this.node = node;
        this.render();
        return true;
    }

    stopEvent(event: Event): boolean {
        return event.target instanceof globalThis.Node && !!this.actions?.contains(event.target);
    }

    ignoreMutation(): boolean {
        return true;
    }

    show(url: string | null) {
        this.preview = url;
        this.dom.closest(".image-block")?.classList.toggle("is-uploading", url !== null);
        this.render();
    }

    private render() {
        const shown = this.preview ?? this.node.attrs.src;
        if (this.img.getAttribute("src") !== shown) this.img.src = shown;
    }

    private figure(): { pos: number; node: Node } | null {
        const pos = this.getPos();
        if (pos == null) return null;
        const $pos = this.view.state.doc.resolve(pos);
        if ($pos.parent.type !== schema.nodes.figure) return null;
        return { pos: $pos.before(), node: $pos.parent };
    }

    private act(action: string) {
        const figure = this.figure();
        if (!figure) return;

        switch (action) {
            case "replace":
                this.options.pickFiles((files) => {
                    if (files[0]) this.options.uploads.replace(this.view, this.getPos, files[0], this);
                }, false);
                break;
            case "width":
                this.view.dispatch(
                    this.view.state.tr.setNodeMarkup(figure.pos, null, {
                        ...figure.node.attrs,
                        width: figure.node.attrs.width === "full" ? "normal" : "full",
                    }),
                );
                break;
            case "remove":
                this.view.dispatch(this.view.state.tr.delete(figure.pos, figure.pos + figure.node.nodeSize));
                this.view.focus();
                break;
        }
    }
}
