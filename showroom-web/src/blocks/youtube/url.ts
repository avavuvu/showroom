import type { YoutubeAttrs } from "../../../bindings/types/YoutubeAttrs"
import type { YoutubeDetails } from "../../../bindings/types/YoutubeDetails"
import { postJson } from "../lookup"

export type { YoutubeAttrs, YoutubeDetails }

export interface YoutubeLink {
    id: string
    start: number | null
    isShort: boolean
}

const VIDEO_ID = /^[A-Za-z0-9_-]{11}$/
const MAX_START = 12 * 60 * 60
const VIDEO_HOSTS = new Set(["youtube.com", "www.youtube.com", "m.youtube.com", "youtube-nocookie.com", "www.youtube-nocookie.com"])
const SHORT_HOSTS = new Set(["youtu.be", "www.youtu.be"])
const PATH_PREFIXES = new Set(["embed", "shorts", "live", "v"])

const parseStart = (value: string | null): number | null => {
    if (!value) {
        return null
    }
    const match = /^(?:(\d+)h)?(?:(\d+)m)?(?:(\d+)s?)?$/.exec(value)
    if (!match) {
        return null
    }
    const [, hours = "0", minutes = "0", seconds = "0"] = match
    const start = Number(hours) * 3600 + Number(minutes) * 60 + Number(seconds)
    return start > 0 && start <= MAX_START ? start : null
}

const findVideoId = (url: URL): string | null => {
    const host = url.hostname.toLowerCase()
    const [first, second] = url.pathname.split("/").filter(Boolean)
    if (SHORT_HOSTS.has(host)) {
        return first ?? null
    }
    if (!VIDEO_HOSTS.has(host)) {
        return null
    }
    if (first === "watch") {
        return url.searchParams.get("v")
    }
    return first && PATH_PREFIXES.has(first) ? second ?? null : null
}

export const parseYoutubeUrl = (href: string): YoutubeLink | null => {
    let url: URL
    try {
        url = new URL(href)
    } catch {
        return null
    }
    if (url.protocol !== "https:" && url.protocol !== "http:") {
        return null
    }
    const id = findVideoId(url)
    if (!id || !VIDEO_ID.test(id)) {
        return null
    }
    return {
        id,
        start: parseStart(url.searchParams.get("t") ?? url.searchParams.get("start")),
        isShort: url.pathname.startsWith("/shorts/"),
    }
}

export const getThumbnailUrl = ({ id, hd }: Pick<YoutubeAttrs, "id" | "hd">): string =>
    `https://i.ytimg.com/vi/${id}/${hd ? "maxresdefault" : "hqdefault"}.jpg`

export const getWatchUrl = ({ id, start }: Pick<YoutubeAttrs, "id" | "start">): string =>
    `https://www.youtube.com/watch?v=${id}${start ? `&t=${start}s` : ""}`

export const lookupVideo = async (id: string): Promise<YoutubeDetails> => {
    const result = await postJson<YoutubeDetails>("/youtube/lookup", { id })
    if (result.kind === "expired") {
        throw new Error("Your session ended. Log in again to load the video details.")
    }
    if (result.kind === "failed") {
        throw new Error(result.error ?? "The video details could not be looked up.")
    }
    return result.body
}
