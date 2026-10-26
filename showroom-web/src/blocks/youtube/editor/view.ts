import { blockView } from "../../view"
import { type YoutubeAttrs, lookupVideo } from "../url"
import { type YoutubeActions, youtubeEditorKey } from "./handle"

export const youtubeView = (template: HTMLTemplateElement) => blockView<YoutubeAttrs, YoutubeActions>({
    template,
    key: youtubeEditorKey,
    actions: (block) => {
        let hasLookedUp = false
        return {
            lookup: async () => {
                if (hasLookedUp || block.attrs.title || block.attrs.channel) {
                    return
                }
                hasLookedUp = true
                try {
                    const details = await block.run("Looking up the video…", () => lookupVideo(block.attrs.id))
                    if (details) {
                        block.set({ ...block.attrs, ...details }, { isUndoable: false })
                    }
                } catch (error) {
                    block.showNotice(error instanceof Error ? error.message : "The video details could not be looked up.")
                }
            },
        }
    },
})
