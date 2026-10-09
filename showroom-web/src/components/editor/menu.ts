import { visibleBounds } from "./viewport";

interface PopupMenuOptions {
    button: HTMLButtonElement;
    panel: HTMLElement;
    items: HTMLButtonElement[];
    onChoose: (item: HTMLButtonElement) => void;
    onEscape?: () => void;
    current?: () => HTMLButtonElement | undefined;
    anchor?: HTMLElement;
    signal: AbortSignal;
}

type Focus = "first" | "last" | "current" | "none";

export class PopupMenu {
    constructor(private options: PopupMenuOptions) {
        const { button, panel, items, signal } = options;

        for (const item of items) {
            item.addEventListener("mousedown", (event) => event.preventDefault(), { signal });
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
        button.addEventListener("click", (event) => (this.isOpen ? this.close(false) : this.open(event.detail === 0 ? "current" : "none")), {
            signal,
        });
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
        window.visualViewport?.addEventListener("resize", () => this.position(), { signal });
        window.visualViewport?.addEventListener("scroll", () => this.position(), { signal });
    }

    get isOpen(): boolean {
        return !this.options.panel.hidden;
    }

    open(focus: Focus = "current") {
        const { button, panel, items } = this.options;
        if (button.disabled) return;
        panel.hidden = false;
        button.setAttribute("aria-expanded", "true");
        this.position();

        if (focus === "none") return;
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
        const { top, bottom } = visibleBounds();
        const below = rect.bottom + height <= bottom;
        const above = (this.options.anchor ?? button).getBoundingClientRect().top - height;
        panel.style.left = `${Math.max(0, Math.min(rect.left, window.innerWidth - width))}px`;
        panel.style.top = `${below ? rect.bottom : Math.max(top, above)}px`;
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
