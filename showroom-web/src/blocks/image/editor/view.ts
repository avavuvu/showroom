import type { PickFiles } from "../../../components/editor/file-picker"
import { schema } from "../../../components/editor/prosemirror/schema"
import { type BlockView, blockView } from "../../view"
import type { Uploads } from "../upload"
import { type ImageActions, type ImageAttrs, imageEditorKey } from "./handle"

export interface ImageViewOptions {
    template: HTMLTemplateElement
    pickFiles: PickFiles
    uploads: Uploads
}

const findFigure = (block: BlockView<ImageAttrs, ImageActions>) => {
    const pos = block.getPos()
    if (pos == null) {
        return null
    }
    const $pos = block.view.state.doc.resolve(pos)
    if ($pos.parent.type !== schema.nodes.figure) {
        return null
    }
    return { pos: $pos.before(), node: $pos.parent }
}

export const imageView = ({ template, pickFiles, uploads }: ImageViewOptions) => blockView<ImageAttrs, ImageActions>({
    template,
    key: imageEditorKey,
    actions: (block) => ({
        getPreview: () => uploads.getPreview(block.getDecorations()),

        replace: () => {
            pickFiles(([file]) => {
                const pos = block.getPos()
                if (file && pos != null) {
                    uploads.replace(block.view, pos, file)
                }
            }, false)
        },

        toggleWidth: () => {
            const figure = findFigure(block)
            if (!figure) {
                return
            }
            const width = figure.node.attrs.width === "full" ? "normal" : "full"
            block.view.dispatch(block.view.state.tr.setNodeMarkup(figure.pos, null, { ...figure.node.attrs, width }))
        },

        removeFigure: () => {
            const figure = findFigure(block)
            if (!figure) {
                return
            }
            block.view.dispatch(block.view.state.tr.delete(figure.pos, figure.pos + figure.node.nodeSize))
            block.view.focus()
        },
    }),
})
