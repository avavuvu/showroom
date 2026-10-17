import type { DirtyForm } from "@setups"
import { holdUnload } from "./unload"

const isChanged = (element: Element) => {
    if (element instanceof HTMLInputElement) {
        return element.type === "checkbox" || element.type === "radio"
            ? element.checked !== element.defaultChecked
            : element.value !== element.defaultValue
    }
    if (element instanceof HTMLTextAreaElement) {
        return element.value !== element.defaultValue
    }
    if (element instanceof HTMLSelectElement) {
        return [...element.options].some((option) => option.selected !== option.defaultSelected)
    }
    return false
}

export const dirtyForm: DirtyForm = (form) => {
    let isSubmitting = false

    const isDirty = () => [...form.elements].some(isChanged)
    const update = () => form.toggleAttribute("data-dirty", isDirty())

    update()

    return {
        update,

        reset: () => setTimeout(update),

        submit: () => {
            isSubmitting = true
        },

        leave: (event) => holdUnload(event, !isSubmitting && isDirty()),

        restore: () => {
            isSubmitting = false
            update()
        },
    }
}
