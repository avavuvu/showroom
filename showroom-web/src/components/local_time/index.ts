import type { LocalTime } from "@setups"

export const localTime: LocalTime = (root) => {
    const date = new Date(root.getAttribute("datetime") ?? "")
    if (!Number.isNaN(date.getTime())) {
        root.textContent = date.toLocaleString(undefined, { month: "long", day: "numeric", hour: "numeric", minute: "2-digit" })
    }
    return {}
}
