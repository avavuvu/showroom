import Alpine from "alpinejs";
import "htmx.org";

const asciiContainer = document.getElementById("ascii-background");
if (asciiContainer) {
    import("./ascii/background").then(({ initAsciiBackground }) => {
        initAsciiBackground(asciiContainer);
    });
}

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
