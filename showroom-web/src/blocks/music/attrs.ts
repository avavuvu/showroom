import type { MusicAttrs } from "../../../bindings/types/MusicAttrs";
import type { MusicService } from "../../../bindings/types/MusicService";
import type { Resolved } from "../../../bindings/types/Resolved";

export type { MusicAttrs, MusicService, Resolved };

export const SERVICES: MusicService[] = ["appleMusic", "spotify", "bandcamp", "youtubeMusic"];

export const SERVICE_LABELS: Record<MusicService, string> = {
    appleMusic: "Apple Music",
    spotify: "Spotify",
    bandcamp: "Bandcamp",
    youtubeMusic: "YouTube Music",
};

const DETAIL_SOURCES: MusicService[] = ["appleMusic", "bandcamp"];

export function hasDetails(attrs: MusicAttrs): boolean {
    return attrs.title !== "" || attrs.artist !== "" || !!attrs.artwork;
}

export function linkCount(attrs: MusicAttrs): number {
    return SERVICES.filter((service) => attrs.links[service]).length;
}

export function attrsFrom(resolved: Resolved): MusicAttrs {
    return withLink({ kind: resolved.kind, title: "", artist: "", artwork: null, source: null, links: {} }, resolved);
}

export function withLink(attrs: MusicAttrs, resolved: Resolved): MusicAttrs {
    const next: MusicAttrs = {
        ...attrs,
        links: { ...attrs.links, [resolved.service]: { url: resolved.url, id: resolved.id ?? null } },
    };
    if (resolved.details && DETAIL_SOURCES.includes(resolved.service) && !hasDetails(attrs)) {
        next.title = resolved.details.title;
        next.artist = resolved.details.artist;
        next.artwork = resolved.details.artwork ?? null;
        next.source = resolved.service;
    }
    return next;
}

export function withoutLink(attrs: MusicAttrs, service: MusicService): MusicAttrs {
    const links = { ...attrs.links };
    delete links[service];
    return { ...attrs, links, source: attrs.source === service ? null : attrs.source };
}

export function looksLikeMusicLink(text: string): boolean {
    if (/^spotify:(track|album):[A-Za-z0-9]+$/.test(text)) return true;
    let url: URL;
    try {
        url = new URL(text);
    } catch {
        return false;
    }
    if (url.protocol !== "https:" && url.protocol !== "http:") return false;
    const host = url.hostname;
    return (
        host === "open.spotify.com" ||
        host === "music.apple.com" ||
        host === "geo.music.apple.com" ||
        host === "music.youtube.com" ||
        (host.endsWith(".bandcamp.com") && /^\/(track|album)\//.test(url.pathname))
    );
}

export async function resolveMusic(url: string): Promise<Resolved> {
    const response = await fetch("/music/resolve", {
        method: "POST",
        redirect: "manual",
        headers: { "Content-Type": "application/json", Accept: "application/json" },
        body: JSON.stringify({ url }),
    });
    if (response.type === "opaqueredirect") throw new Error("Your session ended. Log in again to add music.");
    const body = (await response.json().catch(() => ({}))) as Partial<Resolved> & { error?: string };
    if (!response.ok) throw new Error(body.error ?? "That link could not be looked up.");
    return body as Resolved;
}

const kindName = (kind: MusicAttrs["kind"]) => (kind === "album" ? "an album" : "a song");

export function lookupProblem(resolved: Resolved, attrs: MusicAttrs | null): string | null {
    if (attrs && resolved.kind !== attrs.kind) {
        return `That ${SERVICE_LABELS[resolved.service]} link is ${kindName(resolved.kind)}, but this card is ${kindName(attrs.kind)}.`;
    }
    if (attrs && hasDetails(attrs)) return null;
    if (DETAIL_SOURCES.includes(resolved.service)) {
        return resolved.details ? null : `The ${SERVICE_LABELS[resolved.service]} details could not be found. Type them in instead.`;
    }
    return "Add an Apple Music or Bandcamp link to fill in the details, or type them in.";
}
