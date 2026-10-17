import type { Node } from "prosemirror-model"
import type { Command } from "prosemirror-state"
import { insertPoint } from "prosemirror-transform"
import { Decoration, type EditorView } from "prosemirror-view"
import type { UploadSignature } from "../../../bindings/types/UploadSignature"
import type { PickFiles } from "../../components/editor/file-picker"
import { canInsert } from "../../components/editor/prosemirror/commands"
import { schema } from "../../components/editor/prosemirror/schema"
import { TrackedDecorations, cloneTemplate, createWidget } from "../decorations"

export interface UploadedImage {
    src: string
    publicId: string
}

const TYPES = ["image/png", "image/jpeg", "image/gif", "image/webp"]
const MAX_BYTES = 10 * 1024 * 1024
const REVOKE_MS = 1_000
const SESSION_ENDED = "Your session ended. Log in again to upload images."

export const insertImage = (uploads: Uploads, pickFiles: PickFiles): Command => (state, dispatch, view) => {
    if (!canInsert(state, schema.nodes.figure)) {
        return false
    }
    if (dispatch && view) {
        pickFiles((files) => uploads.insert(view, files), true)
    }
    return true
}

const findInsertionPoint = (doc: Node, pos: number): number | null => {
    const { figure } = schema.nodes
    const point = insertPoint(doc, pos, figure)
    if (point != null) {
        return point
    }
    const $pos = doc.resolve(pos)
    for (let depth = $pos.depth - 1; depth >= 0; depth--) {
        const index = $pos.indexAfter(depth)
        if ($pos.node(depth).canReplaceWith(index, index, figure)) {
            return $pos.after(depth + 1)
        }
    }
    return null
}

const createFigure = (image: UploadedImage) => {
    const { figure, image: imageType, caption } = schema.nodes
    return figure.create(null, [imageType.create({ src: image.src, publicId: image.publicId }), caption.create()])
}

export const filterImageFiles = (list: FileList | null | undefined): Array<File> =>
    Array.from(list ?? []).filter((file) => file.type.startsWith("image/"))

const getFileProblem = (file: File): string | null => {
    if (!TYPES.includes(file.type)) {
        return `${file.name} is not a PNG, JPEG, GIF or WebP image.`
    }
    if (file.size > MAX_BYTES) {
        return `${file.name} is larger than 10 MB.`
    }
    return null
}

const uploadImage = async (file: File): Promise<UploadedImage> => {
    const signResponse = await fetch("/images/sign", { redirect: "manual", headers: { Accept: "application/json" } })
    if (signResponse.type === "opaqueredirect" || signResponse.status === 401 || signResponse.status === 403) {
        throw new Error(SESSION_ENDED)
    }
    if (!signResponse.ok) {
        throw new Error("The upload could not start.")
    }
    const signature = (await signResponse.json()) as UploadSignature

    const form = new FormData()
    form.append("file", file)
    form.append("api_key", signature.apiKey)
    form.append("timestamp", String(signature.timestamp))
    form.append("signature", signature.signature)
    form.append("folder", signature.folder)
    form.append("allowed_formats", signature.allowedFormats)

    const response = await fetch(`https://api.cloudinary.com/v1_1/${encodeURIComponent(signature.cloudName)}/image/upload`, {
        method: "POST",
        body: form,
    })
    const body = (await response.json().catch(() => ({}))) as { secure_url?: unknown, public_id?: unknown, error?: { message?: string } }
    if (!response.ok || typeof body.secure_url !== "string" || typeof body.public_id !== "string") {
        throw new Error(body.error?.message ?? `The upload failed (${response.status}).`)
    }

    const loaded = new Image()
    loaded.src = body.secure_url
    await loaded.decode().catch(() => undefined)

    return { src: body.secure_url, publicId: body.public_id }
}

export class Uploads {
    private activeCount = 0
    private decorations = new TrackedDecorations("upload")

    constructor(
        private template: HTMLTemplateElement,
        private report: (message: string) => void,
    ) {}

    get isActive(): boolean {
        return this.activeCount > 0
    }

    plugin() {
        return this.decorations.plugin()
    }

    getPreview(decorations: ReadonlyArray<Decoration>): string | null {
        const found = decorations.find((decoration) => typeof decoration.spec.preview === "string")
        return found ? found.spec.preview : null
    }

    insert(view: EditorView, files: Array<File>, pos?: number) {
        for (const file of files) {
            if (!this.accepts(file)) {
                continue
            }

            const state = view.state
            const at = findInsertionPoint(state.doc, pos ?? state.selection.from)
            if (at == null) {
                this.report("An image cannot go here.")
                continue
            }
            const id = {}
            const preview = URL.createObjectURL(file)
            view.dispatch(this.decorations.change(state.tr, { add: createWidget(id, at, () => this.renderPlaceholder(preview)) }))

            void this.run(view, file, preview, (image) => {
                const placeholder = this.decorations.find(view.state, id)
                const transaction = this.decorations.change(view.state.tr, { remove: id })
                if (image) {
                    const target = placeholder ? findInsertionPoint(transaction.doc, placeholder.from) : null
                    if (target != null) {
                        transaction.insert(target, createFigure(image))
                    } else {
                        this.report("An image finished uploading, but the place it was going was deleted.")
                    }
                }
                view.dispatch(transaction)
            })
        }
    }

    replace(view: EditorView, pos: number, file: File) {
        if (!this.accepts(file)) {
            return
        }

        const node = view.state.doc.nodeAt(pos)
        if (node?.type !== schema.nodes.image) {
            return
        }
        const id = {}
        const preview = URL.createObjectURL(file)
        view.dispatch(this.decorations.change(view.state.tr, { add: Decoration.node(pos, pos + node.nodeSize, {}, { id, preview }) }))

        void this.run(view, file, preview, (image) => {
            const found = this.decorations.find(view.state, id)
            const transaction = this.decorations.change(view.state.tr, { remove: id })
            const target = found ? transaction.doc.nodeAt(found.from) : null
            if (image && found && target?.type === schema.nodes.image) {
                transaction.setNodeMarkup(found.from, null, { ...target.attrs, src: image.src, publicId: image.publicId })
            } else if (image) {
                this.report("An image finished uploading, but the image it was replacing was deleted.")
            }
            view.dispatch(transaction)
        })
    }

    upload(file: File): Promise<UploadedImage | null> {
        if (!this.accepts(file)) {
            return Promise.resolve(null)
        }
        return this.attempt(file)
    }

    private accepts(file: File): boolean {
        const problem = getFileProblem(file)
        if (problem) {
            this.report(problem)
        }
        return problem == null
    }

    private renderPlaceholder(preview: string): HTMLElement {
        const element = cloneTemplate(this.template)
        const image = element.querySelector("img")
        if (image) {
            image.src = preview
        }
        return element
    }

    private async run(view: EditorView, file: File, preview: string, done: (image: UploadedImage | null) => void) {
        const image = await this.attempt(file)
        setTimeout(() => URL.revokeObjectURL(preview), REVOKE_MS)
        if (!view.isDestroyed) {
            done(image)
        }
    }

    private async attempt(file: File): Promise<UploadedImage | null> {
        this.activeCount++
        try {
            return await uploadImage(file)
        } catch (error) {
            this.report(error instanceof Error ? error.message : "The upload failed.")
            return null
        } finally {
            this.activeCount--
        }
    }
}
