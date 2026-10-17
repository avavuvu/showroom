import { context } from "../../../../bindings/boutique/setups"
import type { BlockHandle } from "../../view"

export interface ImageAttrs {
    src: string
    title: string | null
    publicId: string | null
}

export interface ImageActions {
    getPreview(): string | null
    replace(): void
    toggleWidth(): void
    removeFigure(): void
}

export type ImageHandle = BlockHandle<ImageAttrs> & ImageActions

export const imageEditorKey = context<ImageHandle>("image-editor")
