import { context } from "../../../../bindings/boutique/setups"
import type { BlockHandle } from "../../view"
import type { YoutubeAttrs } from "../url"

export interface YoutubeActions {
    lookup(): Promise<void>
}

export type YoutubeHandle = BlockHandle<YoutubeAttrs> & YoutubeActions

export const youtubeEditorKey = context<YoutubeHandle>("youtube-editor")
