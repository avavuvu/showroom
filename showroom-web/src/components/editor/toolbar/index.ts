import { inject } from "../../../../bindings/boutique/setups"
import type { EditorToolbar } from "@setups"
import { editorContext } from "../context"
import { BLOCKS, canChangeBlockType, getCurrentBlockType } from "../prosemirror/commands"
import { formatShortcut } from "../shortcuts"
import { createToolbarCommands } from "./commands"

const isVisible = (element: HTMLElement) => element.getClientRects().length > 0
const isInMenu = (element: Element) => element.closest('[role="menu"]') !== null

export const editorToolbar: EditorToolbar = async (root, { command, blockButton, blockLabel, blockOption }, { signal }) => {
    const { view, onUpdate, insertImage } = await inject(root, editorContext).catch((error: unknown) => {
        for (const button of command) {
            button.disabled = true
        }
        blockButton.disabled = true
        throw error
    })
    const commands = createToolbarCommands(insertImage)

    for (const element of root.querySelectorAll<HTMLElement>("[data-shortcut]")) {
        const text = formatShortcut(element.dataset.shortcut ?? "")
        const label = element.querySelector(".shortcut")
        if (label) {
            label.textContent = text
        } else {
            element.title = `${element.getAttribute("aria-label")} (${text})`
        }
    }

    let current: HTMLButtonElement | null = null

    const getItems = () => [...root.querySelectorAll<HTMLButtonElement>("button")].filter((item) => !isInMenu(item) && !item.disabled && isVisible(item))

    const makeCurrent = (item: HTMLButtonElement | null) => {
        for (const button of root.querySelectorAll("button")) {
            if (!isInMenu(button)) {
                button.tabIndex = button === item ? 0 : -1
            }
        }
        current = item
    }

    const sync = () => {
        for (const button of command) {
            const toolbarCommand = commands[button.dataset.command ?? ""]
            if (!toolbarCommand) {
                continue
            }
            button.disabled = !toolbarCommand.run(view.state, undefined, view)
            if (toolbarCommand.isActive) {
                button.setAttribute(isInMenu(button) ? "aria-checked" : "aria-pressed", String(toolbarCommand.isActive(view.state)))
            }
        }

        const blockType = getCurrentBlockType(view.state)
        for (const option of blockOption) {
            option.setAttribute("aria-checked", String(option.dataset.block === blockType))
        }
        const isKnown = blockLabel.some((label) => label.dataset.block === blockType)
        for (const label of blockLabel) {
            label.hidden = label.dataset.block !== (isKnown ? blockType : "mixed")
        }
        const shownLabel = blockLabel.find((label) => !label.hidden)?.textContent?.trim()
        blockButton.setAttribute("aria-label", shownLabel ? `${blockButton.title}: ${shownLabel}` : blockButton.title)

        blockButton.disabled = !canChangeBlockType(view.state)
        const panel = blockButton.popoverTargetElement as HTMLElement | null
        if (blockButton.disabled && panel?.matches(":popover-open")) {
            panel.hidePopover()
        }

        if (!current || current.disabled || !isVisible(current)) {
            makeCurrent(getItems()[0] ?? null)
        }
    }

    onUpdate(sync, signal)

    return {
        runCommand: (_event, button) => {
            const name = button.dataset.command ?? ""
            commands[name]?.run(view.state, view.dispatch, view)
            if (name !== "link" && name !== "image") {
                view.focus()
            }
        },

        setBlock: (_event, option) => {
            BLOCKS[option.dataset.block ?? ""]?.(view.state, view.dispatch, view)
            view.focus()
        },

        track: (event) => {
            if (event.target instanceof HTMLButtonElement && !isInMenu(event.target)) {
                makeCurrent(event.target)
            }
        },

        navigate: (event) => {
            if (event.defaultPrevented || (event.target instanceof Element && isInMenu(event.target))) {
                return
            }

            const items = getItems()
            const index = current ? items.indexOf(current) : -1
            const next = {
                ArrowRight: items[(index + 1) % items.length],
                ArrowLeft: items[(index - 1 + items.length) % items.length],
                Home: items[0],
                End: items.at(-1),
            }[event.key]

            if (event.key === "Escape") {
                event.preventDefault()
                view.focus()
                return
            }
            if (!next) {
                return
            }
            event.preventDefault()
            makeCurrent(next)
            next.focus()
        },
    }
}
