import Alpine from "alpinejs";

Alpine.data("saveStatus", () => ({
    status: "Saved",
    init() {
        window.addEventListener("save-status", (e) => {
            this.status = (e as CustomEvent).detail;
        });
    },
}));

Alpine.data("form", () => ({
    validate(event: Event) {
        event.preventDefault();
        let valid = true;

        for (const input of (
            this.$el as HTMLFormElement
        ).querySelectorAll<HTMLInputElement>("input[required]")) {
            const error = document.getElementById(`${input.name}-error`);
            if (!input.value.trim()) {
                if (error) error.textContent = "This is required";
                valid = false;
            } else {
                if (error) error.textContent = "";
            }
        }

        if (valid) {
            (this.$el as HTMLElement).dispatchEvent(
                new Event("validated", { bubbles: true }),
            );
        }
    },
}));

Alpine.start();
