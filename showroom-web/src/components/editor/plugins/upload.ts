import { type EditorState, Plugin, PluginKey } from "prosemirror-state";
import { insertPoint } from "prosemirror-transform";
import { Decoration, DecorationSet, type EditorView } from "prosemirror-view";
import { schema } from "../schema";

interface UploadMeta {
    add?: { id: object; pos: number; preview: string };
    remove?: { id: object };
}

export interface UploadPreview {
    show(url: string | null): void;
}

export interface UploadedImage {
    src: string;
    publicId: string;
}

interface Signature {
    signature: string;
    timestamp: number;
    api_key: string;
    cloud_name: string;
    folder: string;
    allowed_formats: string;
}

const uploadKey = new PluginKey<DecorationSet>("upload");
const TYPES = ["image/png", "image/jpeg", "image/gif", "image/webp"];
const MAX_BYTES = 10 * 1024 * 1024;

export function uploadPlugin(template: HTMLTemplateElement) {
    const placeholder = (preview: string) => () => {
        const cloned = template.content.firstElementChild?.cloneNode(true);
        const element = cloned instanceof HTMLElement ? cloned : document.createElement("figure");
        const image = document.createElement("img");
        image.src = preview;
        image.alt = "";
        (element.querySelector(".image-frame") ?? element).prepend(image);
        return element;
    };

    return new Plugin<DecorationSet>({
        key: uploadKey,
        state: {
            init: () => DecorationSet.empty,
            apply(tr, set) {
                let next = set.map(tr.mapping, tr.doc);
                const meta = tr.getMeta(uploadKey) as UploadMeta | undefined;
                if (meta?.add) {
                    next = next.add(tr.doc, [
                        Decoration.widget(meta.add.pos, placeholder(meta.add.preview), { id: meta.add.id, side: -1 }),
                    ]);
                }
                if (meta?.remove) {
                    const id = meta.remove.id;
                    next = next.remove(next.find(undefined, undefined, (spec) => spec.id === id));
                }
                return next;
            },
        },
        props: {
            decorations: (state) => uploadKey.getState(state),
        },
    });
}

function findPlaceholder(state: EditorState, id: object): number | null {
    const found = uploadKey.getState(state)?.find(undefined, undefined, (spec) => spec.id === id) ?? [];
    return found.length > 0 ? found[0].from : null;
}

function insertionPoint(state: EditorState, pos: number): number | null {
    const { figure } = schema.nodes;
    const point = insertPoint(state.doc, pos, figure);
    if (point != null) return point;
    const $pos = state.doc.resolve(pos);
    for (let depth = $pos.depth - 1; depth >= 0; depth--) {
        const index = $pos.indexAfter(depth);
        if ($pos.node(depth).canReplaceWith(index, index, figure)) return $pos.after(depth + 1);
    }
    return null;
}

function createFigure(image: UploadedImage) {
    const { figure, image: imageType, caption } = schema.nodes;
    return figure.create(null, [imageType.create({ src: image.src, publicId: image.publicId }), caption.create()]);
}

export function imageFiles(list: FileList | null | undefined): File[] {
    return Array.from(list ?? []).filter((file) => file.type.startsWith("image/"));
}

function problemWith(file: File): string | null {
    if (!TYPES.includes(file.type)) return `${file.name} is not a PNG, JPEG, GIF or WebP image.`;
    if (file.size > MAX_BYTES) return `${file.name} is larger than 10 MB.`;
    return null;
}

async function uploadImage(file: File): Promise<UploadedImage> {
    const signResponse = await fetch("/images/sign", { redirect: "manual", headers: { Accept: "application/json" } });
    if (signResponse.type === "opaqueredirect") throw new Error("Your session ended. Log in again to upload images.");
    if (!signResponse.ok) throw new Error("The upload could not start.");
    const signature = (await signResponse.json()) as Signature;

    const form = new FormData();
    form.append("file", file);
    form.append("api_key", signature.api_key);
    form.append("timestamp", String(signature.timestamp));
    form.append("signature", signature.signature);
    form.append("folder", signature.folder);
    form.append("allowed_formats", signature.allowed_formats);

    const response = await fetch(`https://api.cloudinary.com/v1_1/${encodeURIComponent(signature.cloud_name)}/image/upload`, {
        method: "POST",
        body: form,
    });
    const body = (await response.json().catch(() => ({}))) as { secure_url?: unknown; public_id?: unknown; error?: { message?: string } };
    if (!response.ok || typeof body.secure_url !== "string" || typeof body.public_id !== "string") {
        throw new Error(body.error?.message ?? `The upload failed (${response.status}).`);
    }

    const loaded = new Image();
    loaded.src = body.secure_url;
    await loaded.decode().catch(() => undefined);

    return { src: body.secure_url, publicId: body.public_id };
}

export class Uploads {
    private count = 0;

    constructor(private report: (message: string) => void) {}

    get active(): boolean {
        return this.count > 0;
    }

    insert(view: EditorView, files: File[], pos?: number) {
        for (const file of files) {
            const problem = problemWith(file);
            if (problem) {
                this.report(problem);
                continue;
            }

            const state = view.state;
            const at = insertionPoint(state, pos ?? state.selection.from);
            if (at == null) {
                this.report("An image cannot go here.");
                continue;
            }
            const id = {};
            const preview = URL.createObjectURL(file);
            view.dispatch(state.tr.setMeta(uploadKey, { add: { id, pos: at, preview } } satisfies UploadMeta));

            void this.run(file, (image) => {
                setTimeout(() => URL.revokeObjectURL(preview), 1_000);
                if (view.isDestroyed) return;
                const placeholder = findPlaceholder(view.state, id);
                const tr = view.state.tr.setMeta(uploadKey, { remove: { id } } satisfies UploadMeta);
                if (image && placeholder != null) {
                    tr.insert(placeholder, createFigure(image));
                }
                view.dispatch(tr);
            });
        }
    }

    replace(view: EditorView, getPos: () => number | undefined, file: File, preview: UploadPreview) {
        const problem = problemWith(file);
        if (problem) {
            this.report(problem);
            return;
        }

        const url = URL.createObjectURL(file);
        preview.show(url);
        void this.run(file, (image) => {
            preview.show(null);
            setTimeout(() => URL.revokeObjectURL(url), 1_000);
            const pos = getPos();
            if (!image || view.isDestroyed || pos == null) return;
            const node = view.state.doc.nodeAt(pos);
            if (node?.type !== schema.nodes.image) return;
            view.dispatch(view.state.tr.setNodeMarkup(pos, null, { ...node.attrs, src: image.src, publicId: image.publicId }));
        });
    }

    private async run(file: File, done: (image: UploadedImage | null) => void) {
        this.count++;
        try {
            const image = await uploadImage(file);
            this.count--;
            done(image);
        } catch (error) {
            this.count--;
            done(null);
            this.report(error instanceof Error ? error.message : "The upload failed.");
        }
    }
}
