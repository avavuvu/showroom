import type { ColorInput } from "@setups"

export const colorInput: ColorInput = (details, { customColor, preset, hex }) => {
    const root = document.documentElement
    const variable = `--${customColor.name}`

    const select = (color: string) => {
        details.style.setProperty("--color", color)
        hex.textContent = color
        root.style.setProperty(variable, color)
        for (const button of preset) {
            button.setAttribute("aria-pressed", String(button.value === color))
        }
    }

    select(customColor.value)

    return {
        change: () => select(customColor.value),

        pick: (_event, button) => {
            customColor.value = button.value
            customColor.dispatchEvent(new Event("input", { bubbles: true }))
        },

        disconnect: () => root.style.removeProperty(variable),
    }
}
