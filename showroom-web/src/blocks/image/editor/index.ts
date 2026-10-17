import { inject } from "../../../../bindings/boutique/setups"
import type { ImageEditor } from "@setups"
import { setImageSource } from "../../view"
import { imageEditorKey } from "./handle"

export const imageEditor: ImageEditor = async (root, { image }, { signal }) => {
    const node = await inject(root, imageEditorKey)

    node.onUpdate(() => {
        const preview = node.getPreview()
        setImageSource(image, preview ?? node.getAttrs().src)
        root.closest(".image-block")?.classList.toggle("is-uploading", preview !== null)
    }, signal)

    return {
        replace: () => node.replace(),

        toggleWidth: () => node.toggleWidth(),

        remove: () => node.removeFigure(),
    }
}
