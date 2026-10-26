import type { MusicAttrs } from "../../../bindings/types/MusicAttrs"
import type { MusicService } from "../../../bindings/types/MusicService"
import type { ArtworkColor } from "../../../bindings/types/ArtworkColor"
import type { Resolved } from "../../../bindings/types/Resolved"
import { postJson } from "../lookup"

export type { MusicAttrs, MusicService, Resolved }

export const SERVICES: Array<MusicService> = ["appleMusic", "spotify", "bandcamp", "youtubeMusic"]

export const SERVICE_LABELS: Record<MusicService, string> = {
    appleMusic: "Apple Music",
    spotify: "Spotify",
    bandcamp: "Bandcamp",
    youtubeMusic: "YouTube Music",
}

const DETAIL_SOURCES: Array<MusicService> = ["appleMusic", "bandcamp"]

export const hasDetails = (attrs: MusicAttrs): boolean =>
    attrs.title !== "" || attrs.artist !== "" || !!attrs.artwork

export const countLinks = (attrs: MusicAttrs): number =>
    SERVICES.filter((service) => attrs.links[service]).length

export const mergeLink = (attrs: MusicAttrs, resolved: Resolved): MusicAttrs => {
    const next: MusicAttrs = {
        ...attrs,
        links: { ...attrs.links, [resolved.service]: { url: resolved.url, id: resolved.id ?? null } },
    }
    if (resolved.details && DETAIL_SOURCES.includes(resolved.service) && !hasDetails(attrs)) {
        next.title = resolved.details.title
        next.artist = resolved.details.artist
        next.artwork = resolved.details.artwork ?? null
        next.color = resolved.details.color ?? null
        next.source = resolved.service
    }
    return next
}

export const createAttrs = (resolved: Resolved): MusicAttrs =>
    mergeLink({ kind: resolved.kind, title: "", artist: "", artwork: null, source: null, links: {}, color: null }, resolved)

export const omitLink = (attrs: MusicAttrs, service: MusicService): MusicAttrs => {
    const links = { ...attrs.links }
    delete links[service]
    return { ...attrs, links, source: attrs.source === service ? null : attrs.source }
}

export const looksLikeMusicLink = (text: string): boolean => {
    if (/^spotify:(track|album):[A-Za-z0-9]+$/.test(text)) {
        return true
    }
    let url: URL
    try {
        url = new URL(text)
    } catch {
        return false
    }
    if (url.protocol !== "https:" && url.protocol !== "http:") {
        return false
    }
    const host = url.hostname
    return (
        host === "open.spotify.com" ||
        host === "music.apple.com" ||
        host === "geo.music.apple.com" ||
        host === "music.youtube.com" ||
        (host.endsWith(".bandcamp.com") && /^\/(track|album)\//.test(url.pathname))
    )
}

export const resolveMusic = async (url: string): Promise<Resolved> => {
    const result = await postJson<Resolved>("/music/resolve", { url })
    if (result.kind === "expired") {
        throw new Error("Your session ended. Log in again to add music.")
    }
    if (result.kind === "failed") {
        throw new Error(result.error ?? "That link could not be looked up.")
    }
    return result.body
}

export const fetchArtworkColor = async (artwork: string): Promise<string | null> => {
    const result = await postJson<ArtworkColor>("/music/color", { artwork })
    return result.kind === "ok" ? result.body.color : null
}

const describeKind = (kind: MusicAttrs["kind"]) => (kind === "album" ? "an album" : "a song")

export const getLookupProblem = (resolved: Resolved, attrs: MusicAttrs | null): string | null => {
    if (attrs && resolved.kind !== attrs.kind) {
        return `That ${SERVICE_LABELS[resolved.service]} link is ${describeKind(resolved.kind)}, but this card is ${describeKind(attrs.kind)}.`
    }
    if (attrs && hasDetails(attrs)) {
        return null
    }
    if (DETAIL_SOURCES.includes(resolved.service)) {
        return resolved.details ? null : `The ${SERVICE_LABELS[resolved.service]} details could not be found. You can still add them manually.`
    }
    return "Add an Apple Music or Bandcamp link to fill in the details, or type them in."
}
