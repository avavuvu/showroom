import type { YoutubeEmbed } from "@setups"

const ORIGINS = ["https://www.youtube-nocookie.com", "https://www.google.com", "https://s.ytimg.com"]
const PERMISSIONS = "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"

let isWarm = false

const warmConnections = () => {
    if (isWarm) {
        return
    }
    isWarm = true
    for (const origin of ORIGINS) {
        const link = document.createElement("link")
        link.rel = "preconnect"
        link.href = origin
        document.head.append(link)
    }
}

export const youtubeEmbed: YoutubeEmbed = (root, refs) => ({
    warm: () => warmConnections(),

    play: () => {
        const src = root.dataset.embed
        if (!src) {
            return
        }
        const iframe = document.createElement("iframe")
        iframe.src = src
        iframe.title = "YouTube video"
        iframe.allow = PERMISSIONS
        iframe.allowFullscreen = true
        iframe.referrerPolicy = "strict-origin-when-cross-origin"
        refs.poster.replaceWith(iframe)
        root.classList.add("is-playing")
        iframe.focus()
    },
})
