import { inject } from "../../../../bindings/boutique/setups"
import type { YoutubeEditor } from "@setups"
import { setCardSurface, setImageSource } from "../../view"
import { getThumbnailUrl, getWatchUrl } from "../url"
import { youtubeEditorKey } from "./handle"

const setText = (element: HTMLElement, text: string) => {
    element.textContent = text
    element.hidden = !text
}

export const youtubeEditor: YoutubeEditor = async (root, refs, { signal }) => {
    const node = await inject(root, youtubeEditorKey)

    node.onUpdate(() => {
        const attrs = node.getAttrs()
        const status = node.getStatus()
        root.classList.toggle("is-short", attrs.short)
        setCardSurface(root, attrs.color)
        setImageSource(refs.thumbnail, getThumbnailUrl(attrs))
        refs.open.href = getWatchUrl(attrs)
        setText(refs.title, attrs.title)
        setText(refs.channel, attrs.channel)
        setText(refs.status, status)
        refs.details.hidden = !attrs.title && !attrs.channel && !status
    }, signal)

    void node.lookup()

    return {
        remove: () => node.remove(),
    }
}
