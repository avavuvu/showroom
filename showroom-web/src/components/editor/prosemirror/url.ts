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
