import { context } from "../../../../bindings/boutique/setups"
import type { BlockHandle } from "../../view"
import type { MusicAttrs, MusicService } from "../attrs"

export interface MusicActions {
    addLink(text: string): Promise<boolean>
    pickCover(): void
    unlink(service: MusicService): void
}

export type MusicHandle = BlockHandle<MusicAttrs> & MusicActions

export const musicEditorKey = context<MusicHandle>("music-editor")
