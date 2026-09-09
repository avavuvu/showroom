import Alpine from "alpinejs";

declare global {
    interface Window {
        Alpine: typeof Alpine;
    }
}

// hx-alpine-compat looks for alpine on window
window.Alpine = Alpine;

Alpine.data("saveStatus", () => ({
    status: "Saved",
    init() {
        window.addEventListener("save-status", (e) => {
            this.status = (e as CustomEvent).detail;
        });
    },
}));

Alpine.start();
