import type { ColorOverride } from "@setups";

export const colorOverride: ColorOverride = (_row, { picker, toggle, reset, hex, signal }) => {
    const root = document.documentElement;
    const variable = `--${picker.name}`;

    const render = () => {
        const custom = toggle.checked;
        hex.textContent = custom ? picker.value : "";
        reset.hidden = !custom;
        if (custom) {
            root.style.setProperty(variable, picker.value);
        } else {
            root.style.removeProperty(variable);
        }
    };

    picker.addEventListener(
        "input",
        () => {
            toggle.checked = true;
            render();
        },
        { signal },
    );

    reset.addEventListener(
        "click",
        () => {
            toggle.checked = false;
            picker.value = picker.defaultValue;
            render();
            picker.focus();
            toggle.dispatchEvent(new Event("input", { bubbles: true }));
        },
        { signal },
    );

    signal.addEventListener("abort", () => root.style.removeProperty(variable), { once: true });

    render();
};
