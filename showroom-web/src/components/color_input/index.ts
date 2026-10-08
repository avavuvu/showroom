import type { ColorInput, ThemePreview } from "@setups";

export const colorInput: ColorInput = (details, { customColor, preset, hex, signal }) => {
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

    customColor.addEventListener("input", () => select(customColor.value), { signal });

    for (const button of preset) {
        button.addEventListener(
            "click",
            () => {
                customColor.value = button.value;
                customColor.dispatchEvent(new Event("input", { bubbles: true }));
            },
            { signal },
        );
    }

    signal.addEventListener("abort", () => root.style.removeProperty(variable), { once: true });

    select(customColor.value);
};

export const themePreview: ThemePreview = (form, { signal }) => {
    const style = document.getElementById("theme");
    if (!style) return;

    let timer: ReturnType<typeof setTimeout> | undefined;
    let request: AbortController | undefined;

    const refresh = async () => {
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

    form.addEventListener(
        "input",
        () => {
            clearTimeout(timer);
            timer = setTimeout(refresh, 100);
        },
        { signal },
    );

    signal.addEventListener(
        "abort",
        () => {
            clearTimeout(timer);
            request?.abort();
        },
        { once: true },
    );
};
