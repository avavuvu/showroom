import type { Dropdown } from "@setups";

export const dropdown: Dropdown = (details, { signal }) => {
    document.addEventListener(
        "pointerdown",
        (event) => {
            if (details.open && !details.contains(event.target as Node)) details.open = false;
        },
        { signal },
    );

    details.addEventListener(
        "keydown",
        (event) => {
            if (event.key !== "Escape" || !details.open) return;
            event.preventDefault();
            details.open = false;
            details.querySelector("summary")?.focus();
        },
        { signal },
    );
};
