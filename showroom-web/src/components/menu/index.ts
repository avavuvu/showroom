import type { Menu } from "@setups";

type Focus = "first" | "last" | "current" | "none";

const ITEM = '[role^="menuitem"]';

function visibleBounds(): { top: number; bottom: number } {
    const viewport = window.visualViewport;
    const top = viewport?.offsetTop ?? 0;
    return { top, bottom: top + (viewport?.height ?? window.innerHeight) };
}

export const menu: Menu = (root, { button, panel }, { signal }) => {
    const anchor = root.dataset.anchor ? root.closest<HTMLElement>(root.dataset.anchor) : null;
    const items = () => [...panel.querySelectorAll<HTMLButtonElement>(ITEM)].filter((item) => !item.hidden && !item.disabled);
    const isOpen = () => !panel.hidden;

    const close = (focusButton: boolean) => {
        if (!isOpen()) return;
        panel.hidden = true;
        button.setAttribute("aria-expanded", "false");
        if (focusButton) button.focus();
    };

    const position = () => {
        if (!isOpen()) return;
        if (button.getClientRects().length === 0) {
            close(false);
            return;
        }
        const rect = button.getBoundingClientRect();
        const { top, bottom } = visibleBounds();
        const below = rect.bottom + panel.offsetHeight <= bottom;
        const above = (anchor ?? button).getBoundingClientRect().top - panel.offsetHeight;
        panel.style.left = `${Math.max(0, Math.min(rect.left, window.innerWidth - panel.offsetWidth))}px`;
        panel.style.top = `${below ? rect.bottom : Math.max(top, above)}px`;
    };

    const open = (focus: Focus) => {
        if (button.disabled) return;
        panel.hidden = false;
        button.setAttribute("aria-expanded", "true");
        position();

        const list = items();
        const target =
            focus === "first" ? list[0]
            : focus === "last" ? list[list.length - 1]
            : focus === "current" ? (list.find((item) => item.getAttribute("aria-checked") === "true") ?? list[0])
            : undefined;
        target?.focus();
    };

    window.visualViewport?.addEventListener("resize", position, { signal });
    window.visualViewport?.addEventListener("scroll", position, { signal });

    const disabled = new MutationObserver(() => {
        if (button.disabled) close(false);
    });
    disabled.observe(button, { attributes: true, attributeFilter: ["disabled"] });

    return {
        toggle(event) {
            if (isOpen()) close(false);
            else open(event.detail === 0 ? "current" : "none");
        },

        openFromKey(event) {
            open(event.key === "ArrowUp" ? "last" : "first");
        },

        choose(event) {
            if ((event.target as Element).closest(ITEM)) close(false);
        },

        navigate(event) {
            const list = items();
            const index = list.indexOf(document.activeElement as HTMLButtonElement);
            let next: HTMLButtonElement | undefined;

            switch (event.key) {
                case "ArrowDown":
                    next = list[(index + 1) % list.length];
                    break;
                case "ArrowUp":
                    next = list[(index - 1 + list.length) % list.length];
                    break;
                case "Home":
                    next = list[0];
                    break;
                case "End":
                    next = list[list.length - 1];
                    break;
                case "Escape":
                    event.preventDefault();
                    event.stopPropagation();
                    close(true);
                    return;
                case "Tab":
                    close(false);
                    return;
                default:
                    return;
            }

            event.preventDefault();
            event.stopPropagation();
            next?.focus();
        },

        outside(event) {
            if (isOpen() && !root.contains(event.target as Node)) close(false);
        },

        position,

        disconnect() {
            disabled.disconnect();
        },
    };
};
