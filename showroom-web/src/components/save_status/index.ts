import type { SaveStatus } from "@setups";

export const saveStatus: SaveStatus = (status, { signal }) => {
    window.addEventListener(
        "save-status",
        (event) => {
            status.textContent = (event as CustomEvent<string>).detail;
        },
        { signal },
    );
};
