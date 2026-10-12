import type { DirtyForm } from "@setups";

const isChanged = (element: Element) => {
    if (element instanceof HTMLInputElement) {
        return element.type === "checkbox" || element.type === "radio"
            ? element.checked !== element.defaultChecked
            : element.value !== element.defaultValue;
    }
    if (element instanceof HTMLTextAreaElement) {
        return element.value !== element.defaultValue;
    }
    if (element instanceof HTMLSelectElement) {
        return [...element.options].some((option) => option.selected !== option.defaultSelected);
    }
    return false;
};

export const dirtyForm: DirtyForm = (form) => {
    let submitting = false;

    const isDirty = () => [...form.elements].some(isChanged);
    const update = () => form.toggleAttribute("data-dirty", isDirty());

    update();

    return {
        update,

        reset: () => setTimeout(update),

        submit: () => {
            submitting = true;
        },

        leave(event) {
            if (submitting || !isDirty()) return;
            event.preventDefault();
            event.returnValue = "";
        },

        restore() {
            submitting = false;
            update();
        },
    };
};
