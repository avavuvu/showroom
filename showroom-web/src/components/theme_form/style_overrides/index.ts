import type { ColorOverride } from "@setups"

export const colorOverride: ColorOverride = (_row, { picker, toggle, reset, hex }) => {
    const root = document.documentElement
    const variable = `--${picker.name}`

    const render = () => {
        const isCustom = toggle.checked
        hex.textContent = isCustom ? picker.value : ""
        reset.hidden = !isCustom
        if (isCustom) {
            root.style.setProperty(variable, picker.value)
        } else {
            root.style.removeProperty(variable)
        }
    }

    render()

    return {
        change: () => {
            toggle.checked = true
            render()
        },

        clear: () => {
            toggle.checked = false
            picker.value = picker.defaultValue
            render()
            picker.focus()
            toggle.dispatchEvent(new Event("input", { bubbles: true }))
        },

        disconnect: () => root.style.removeProperty(variable),
    }
}
