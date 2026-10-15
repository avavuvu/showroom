import type { Menu } from "@setups";

type Focus = "first" | "last" | "current" | "none";

const ITEM = '[role^="menuitem"]';

export const menu: Menu = (_root, { button, panel }) => {
    let focus: Focus = "none";

    const items = () => [...panel.querySelectorAll<HTMLButtonElement>(ITEM)].filter((item) => !item.hidden && !item.disabled);

    const focusItem = (which: Focus) => {
        const list = items();
        const target =
            which === "first" ? list[0]
            : which === "last" ? list.at(-1)
            : which === "current" ? (list.find((item) => item.getAttribute("aria-checked") === "true") ?? list[0])
            : undefined;
        target?.focus();
    };

    return {
        remember(event) {
            focus = event.detail === 0 ? "current" : "none";
        },

        openFromKey(event) {
            focus = event.key === "ArrowUp" ? "last" : "first";
            if (panel.matches(":popover-open")) focusItem(focus);
            else panel.showPopover();
        },

        opened(event) {
            if (event.newState === "open") focusItem(focus);
            focus = "none";
        },

        choose(event) {
            if ((event.target as Element).closest(ITEM)) panel.hidePopover();
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
                    next = list.at(-1);
                    break;
                case "Escape":
                    event.preventDefault();
                    event.stopPropagation();
                    panel.hidePopover();
                    button.focus();
                    return;
                case "Tab":
                    panel.hidePopover();
                    return;
                default:
                    return;
            }

            event.preventDefault();
            event.stopPropagation();
            next?.focus();
        },
    };
};
