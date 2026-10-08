interface PopupMenuOptions {
    button: HTMLButtonElement;
    panel: HTMLElement;
    items: HTMLButtonElement[];
    onChoose: (item: HTMLButtonElement) => void;
    onEscape?: () => void;
    current?: () => HTMLButtonElement | undefined;
    signal: AbortSignal;
}

export class PopupMenu {
    constructor(private options: PopupMenuOptions) {
        const { button, panel, items, signal } = options;

        for (const item of items) {
            item.addEventListener(
                "click",
                () => {
                    this.close(false);
                    options.onChoose(item);
                },
                { signal },
            );
        }

        button.addEventListener("mousedown", (event) => event.preventDefault(), { signal });
        button.addEventListener("click", () => (this.isOpen ? this.close(false) : this.open()), { signal });
        button.addEventListener(
            "keydown",
            (event) => {
                if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;
                event.preventDefault();
                event.stopPropagation();
                this.open(event.key === "ArrowUp" ? "last" : "first");
            },
            { signal },
        );

        panel.addEventListener("keydown", (event) => this.navigate(event), { signal });
        document.addEventListener(
            "pointerdown",
            (event) => {
                const target = event.target as Node;
                if (this.isOpen && !panel.contains(target) && !button.contains(target)) this.close(false);
            },
            { signal },
        );
        window.addEventListener("resize", () => this.position(), { signal });
        window.addEventListener("scroll", () => this.position(), { signal, passive: true });
    }

    get isOpen(): boolean {
        return !this.options.panel.hidden;
    }

    open(focus: "first" | "last" | "current" = "current") {
        const { button, panel, items } = this.options;
        if (button.disabled) return;
        panel.hidden = false;
        button.setAttribute("aria-expanded", "true");
        this.position();

        const target =
            focus === "last" ? items[items.length - 1] : focus === "first" ? items[0] : (this.options.current?.() ?? items[0]);
        target?.focus();
    }

    close(focusButton: boolean) {
        const { button, panel } = this.options;
        if (!this.isOpen) return;
        panel.hidden = true;
        button.setAttribute("aria-expanded", "false");
        if (focusButton) button.focus();
    }

    position() {
        if (!this.isOpen) return;
        const { button, panel } = this.options;
        const rect = button.getBoundingClientRect();
        const height = panel.offsetHeight;
        const width = panel.offsetWidth;
        const below = rect.bottom + height <= window.innerHeight;
        panel.style.left = `${Math.max(0, Math.min(rect.left, window.innerWidth - width))}px`;
        panel.style.top = `${below ? rect.bottom : Math.max(0, rect.top - height)}px`;
    }

    private navigate(event: KeyboardEvent) {
        const { items } = this.options;
        const index = items.indexOf(document.activeElement as HTMLButtonElement);
        let next: HTMLButtonElement | undefined;

        switch (event.key) {
            case "ArrowDown":
                next = items[(index + 1) % items.length];
                break;
            case "ArrowUp":
                next = items[(index - 1 + items.length) % items.length];
                break;
            case "Home":
                next = items[0];
                break;
            case "End":
                next = items[items.length - 1];
                break;
            case "Escape":
                event.preventDefault();
                event.stopPropagation();
                if (this.options.onEscape) {
                    this.close(false);
                    this.options.onEscape();
                } else {
                    this.close(true);
                }
                return;
            case "Tab":
                this.close(false);
                return;
            default:
                return;
        }

        event.preventDefault();
        event.stopPropagation();
        next?.focus();
    }
}
