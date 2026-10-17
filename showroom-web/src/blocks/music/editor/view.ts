import type { PickFiles } from "../../../components/editor/file-picker"
import type { Uploads } from "../../image/upload"
import { blockView } from "../../view"
import { type MusicAttrs, countLinks, fetchArtworkColor, getLookupProblem, mergeLink, omitLink, resolveMusic } from "../attrs"
import { type MusicActions, musicEditorKey } from "./handle"

export interface MusicViewOptions {
    template: HTMLTemplateElement
    pickFiles: PickFiles
    uploads: Uploads
}

export const musicView = ({ template, pickFiles, uploads }: MusicViewOptions) => blockView<MusicAttrs, MusicActions>({
    template,
    key: musicEditorKey,
    actions: (block) => ({
        addLink: async (text) => {
            const url = text.trim()
            if (!url) {
                return false
            }
            try {
                const resolved = await block.run("Looking up\u2026", () => resolveMusic(url))
                if (!resolved) {
                    return false
                }
                const problem = getLookupProblem(resolved, block.attrs)
                block.set(mergeLink(block.attrs, resolved))
                if (problem) {
                    block.showNotice(problem)
                }
                return true
            } catch (error) {
                block.showNotice(error instanceof Error ? error.message : "That link could not be looked up.")
                return false
            }
        },

        pickCover: () => {
            pickFiles(async ([file]) => {
                if (!file) {
                    return
                }
                const cover = await block.run("Uploading cover\u2026", async () => {
                    const image = await uploads.upload(file)
                    return image && { artwork: image.src, color: await fetchArtworkColor(image.src) }
                })
                if (cover) {
                    block.change(cover)
                }
            }, false)
        },

        unlink: (service) => {
            if (countLinks(block.attrs) > 1) {
                block.set(omitLink(block.attrs, service))
            }
        },
    }),
})
