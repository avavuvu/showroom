import type { Menu } from "@setups"

type Focus = "first" | "last" | "current" | "none"

const ITEM = '[role^="menuitem"]'

export const menu: Menu = (_root, { button, panel }) => {
    let focus: Focus = "none"

    const getItems = () => [...panel.querySelectorAll<HTMLButtonElement>(ITEM)].filter((item) => !item.hidden && !item.disabled)

    const focusItem = (which: Focus) => {
        const items = getItems()
        switch (which) {
            case "first":
                items[0]?.focus()
                return
            case "last":
                items.at(-1)?.focus()
                return
            case "current":
                (items.find((item) => item.getAttribute("aria-checked") === "true") ?? items[0])?.focus()
                return
            case "none":
                return
        }
    }

    return {
        remember: (event) => {
            focus = event.detail === 0 ? "current" : "none"
        },

        openFromKey: (event) => {
            focus = event.key === "ArrowUp" ? "last" : "first"
            if (panel.matches(":popover-open")) {
                focusItem(focus)
            } else {
                panel.showPopover()
            }
        },

        opened: (event) => {
            if (event.newState === "open") {
                focusItem(focus)
            }
            focus = "none"
        },

        choose: (event) => {
            if ((event.target as Element).closest(ITEM)) {
                panel.hidePopover()
            }
        },

        navigate: (event) => {
            const items = getItems()
            const index = items.indexOf(document.activeElement as HTMLButtonElement)
            let next: HTMLButtonElement | undefined

            switch (event.key) {
                case "ArrowDown":
                    next = items[(index + 1) % items.length]
                    break
                case "ArrowUp":
                    next = items[(index - 1 + items.length) % items.length]
                    break
                case "Home":
                    next = items[0]
                    break
                case "End":
                    next = items.at(-1)
                    break
                case "Escape":
                    event.preventDefault()
                    event.stopPropagation()
                    panel.hidePopover()
                    button.focus()
                    return
                case "Tab":
                    panel.hidePopover()
                    return
                default:
                    return
            }

            event.preventDefault()
            event.stopPropagation()
            next?.focus()
        },
    }
}
