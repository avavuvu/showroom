const LINK_SCHEMES = ["http:", "https:", "mailto:", "tel:"]

const parseUrl = (value: string): URL | null => {
    try {
        return new URL(value)
    } catch {
        return null
    }
}

const isWebUrl = (url: URL) => url.protocol === "http:" || url.protocol === "https:"

export const isAllowedHref = (href: string): boolean => {
    const url = parseUrl(href.trim())
    if (!url || !LINK_SCHEMES.includes(url.protocol)) {
        return false
    }
    if (isWebUrl(url)) {
        return url.hostname !== ""
    }
    return url.pathname !== ""
}

export const isAllowedImageSrc = (src: string): boolean => {
    const url = parseUrl(src.trim())
    return !!url && url.protocol === "https:" && url.hostname !== ""
}

export const normalizeHref = (input: string): string | null => {
    const value = input.trim()
    if (!value) {
        return null
    }

    let candidate: string
    if (/^[^@\s/:]+@[^@\s/]+\.[^@\s/]+$/.test(value)) {
        candidate = `mailto:${value}`
    } else if (/^\+?[\d\s().-]{6,}$/.test(value)) {
        candidate = `tel:${value.replace(/[^\d+]/g, "")}`
    } else if (/^[a-z][a-z\d+.-]*:/i.test(value)) {
        candidate = value
    } else {
        candidate = `https://${value.replace(/^\/+/, "")}`
    }

    const url = parseUrl(candidate)
    if (!url || !LINK_SCHEMES.includes(url.protocol)) {
        return null
    }
    if (isWebUrl(url) && !url.hostname.includes(".") && url.hostname !== "localhost") {
        return null
    }
    if (!isAllowedHref(candidate)) {
        return null
    }
    return /\s/.test(candidate) ? url.href : candidate
}

export const displayHref = (href: string): string => {
    if (href.startsWith("mailto:")) {
        return href.slice("mailto:".length)
    }
    if (href.startsWith("tel:")) {
        return href.slice("tel:".length)
    }
    return href
}

const COMMON_TLDS = new Set([
    "com", "net", "org", "edu", "gov", "info", "biz", "xyz", "app", "dev", "page", "blog", "news", "site", "online",
    "store", "shop", "tech", "design", "studio", "art", "media", "social", "space", "cloud", "email", "link", "live",
    "club", "agency", "games", "music", "video", "photo", "photos", "gallery", "world", "today", "fyi", "wtf", "lol",
])
const FILE_EXTENSIONS = new Set(["md", "py", "rs", "sh", "pl", "ts", "js", "cs", "rb", "go", "db", "gz", "xz", "kt", "hs", "ml", "ex", "el", "jl", "hh", "mm", "ps"])
const BARE_LINK = /^(www\.)?(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+([a-z]{2,24})(?::\d{1,5})?(?:[/?#]\S*)?$/i

export const findLinkHref = (text: string): string | null => {
    const value = text.trim()
    if (!value || /\s/.test(value)) {
        return null
    }
    if (/^(https?:\/\/|mailto:)/i.test(value)) {
        return normalizeHref(value)
    }
    const match = BARE_LINK.exec(value)
    if (!match) {
        return null
    }
    const [, www, tld = ""] = match
    const topLevel = tld.toLowerCase()
    const isKnownDomain = COMMON_TLDS.has(topLevel) || (topLevel.length === 2 && !FILE_EXTENSIONS.has(topLevel))
    return www || isKnownDomain ? normalizeHref(value) : null
}
