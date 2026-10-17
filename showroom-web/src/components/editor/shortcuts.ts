import { isMac } from "../../../bindings/boutique/setups"

export const formatShortcut = (shortcut: string): string => {
    const parts = shortcut.split("-")
    const key = parts.pop()?.toUpperCase() ?? ""
    const hasModifier = (name: string) => parts.includes(name)

    if (isMac()) {
        return [hasModifier("Ctrl") ? "⌃" : "", hasModifier("Alt") ? "⌥" : "", hasModifier("Shift") ? "⇧" : "", hasModifier("Mod") ? "⌘" : "", key].join("")
    }
    return [hasModifier("Mod") || hasModifier("Ctrl") ? "Ctrl" : "", hasModifier("Alt") ? "Alt" : "", hasModifier("Shift") ? "Shift" : "", key]
        .filter(Boolean)
        .join("+")
}
