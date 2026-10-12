const isMac = /Mac|iPhone|iPad/.test(navigator.platform);

export function formatShortcut(shortcut: string): string {
    const parts = shortcut.split("-");
    const key = parts.pop()?.toUpperCase() ?? "";
    const has = (name: string) => parts.includes(name);

    if (isMac) {
        return [has("Ctrl") ? "⌃" : "", has("Alt") ? "⌥" : "", has("Shift") ? "⇧" : "", has("Mod") ? "⌘" : "", key].join("");
    }
    return [has("Mod") || has("Ctrl") ? "Ctrl" : "", has("Alt") ? "Alt" : "", has("Shift") ? "Shift" : "", key]
        .filter(Boolean)
        .join("+");
}
