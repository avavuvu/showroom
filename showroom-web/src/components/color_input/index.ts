import type { ColorInput, ThemePreview } from "@setups";

export const colorInput: ColorInput = (details, { customColor, preset, hex }) => {
    const root = document.documentElement;
    const variable = `--${customColor.name}`;

    const select = (color: string) => {
        details.style.setProperty("--color", color);
        hex.textContent = color;
        root.style.setProperty(variable, color);
        for (const button of preset) {
            button.setAttribute("aria-pressed", String(button.value === color));
        }
    };

    select(customColor.value);

    return {
        change: () => select(customColor.value),

        pick(_event, button) {
            customColor.value = button.value;
            customColor.dispatchEvent(new Event("input", { bubbles: true }));
        },

        disconnect: () => root.style.removeProperty(variable),
    };
};

export const themePreview: ThemePreview = (form) => {
    const style = document.getElementById("theme");

    let timer: ReturnType<typeof setTimeout> | undefined;
    let request: AbortController | undefined;

    const refresh = async () => {
        if (!style) return;
        request?.abort();
        const controller = (request = new AbortController());
        try {
            const response = await fetch(`${form.action}/preview`, {
                method: "POST",
                body: new URLSearchParams(new FormData(form) as unknown as string[][]),
                signal: controller.signal,
            });
            if (response.ok) style.textContent = await response.text();
        } catch {}
    };

    return {
        schedule() {
            clearTimeout(timer);
            timer = setTimeout(refresh, 100);
        },

        disconnect() {
            clearTimeout(timer);
            request?.abort();
        },
    };
};
