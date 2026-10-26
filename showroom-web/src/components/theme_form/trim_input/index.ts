import type { TrimInput } from "@setups"

const WOOD = "wood"
const NONE = "none"

const describeTrim = (value: string): { label: string, swatch: string, material: string } => {
    if (value === WOOD) {
        return { label: "Wood", swatch: "var(--trim-wood)", material: "var(--trim-wood)" }
    }
    if (value === NONE) {
        return { label: "None", swatch: "var(--color-primary)", material: "linear-gradient(var(--color-primary) 0 0)" }
    }
    return { label: value, swatch: value, material: `linear-gradient(${value} 0 0)` }
}

export const trimInput: TrimInput = (details, { customColor, preset, hex, value }) => {
    const root = document.documentElement

    const select = (trim: string) => {
        const { label, swatch, material } = describeTrim(trim)
        value.value = trim
        hex.textContent = label
        details.style.setProperty("--trim-swatch", swatch)
        root.style.setProperty("--trim-material", material)
        for (const button of preset) {
            button.setAttribute("aria-pressed", String(button.value === trim))
        }
    }

    const commit = (trim: string) => {
        select(trim)
        value.dispatchEvent(new Event("input", { bubbles: true }))
    }

    return {
        change: () => commit(customColor.value),

        pick: (_event, button) => {
            if (button.value !== WOOD && button.value !== NONE) {
                customColor.value = button.value
            }
            commit(button.value)
        },

        disconnect: () => root.style.removeProperty("--trim-material"),
    }
}
