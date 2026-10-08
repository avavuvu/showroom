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

export const dirtyForm: DirtyForm = (form, { signal }) => {
    let submitting = false;

    const isDirty = () => [...form.elements].some(isChanged);
    const update = () => form.toggleAttribute("data-dirty", isDirty());

    form.addEventListener("input", update, { signal });
    form.addEventListener("change", update, { signal });
    form.addEventListener("reset", () => setTimeout(update), { signal });
    form.addEventListener("submit", () => (submitting = true), { signal });

    window.addEventListener(
        "beforeunload",
        (event) => {
            if (submitting || !isDirty()) return;
            event.preventDefault();
            event.returnValue = "";
        },
        { signal },
    );

    window.addEventListener("pageshow", () => {
        submitting = false;
        update();
    }, { signal });

    update();
};
