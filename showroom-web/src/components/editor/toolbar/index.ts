import { inject } from "@bq/components/src/setups";
import type { EditorToolbar } from "@setups";
import { editorContext } from "../context";
import { BLOCKS, canChangeBlockType, currentBlockType } from "../prosemirror/commands";
import { formatShortcut } from "../shortcuts";
import { toolbarCommands } from "./commands";

const isVisible = (element: HTMLElement) => element.getClientRects().length > 0;
const inMenu = (element: Element) => element.closest('[role="menu"]') !== null;

export const editorToolbar: EditorToolbar = async (root, { command, blockButton, blockLabel, blockOption }, { signal }) => {
    const { view, onUpdate, insertImage } = await inject(root, editorContext).catch((error: unknown) => {
        for (const button of command) button.disabled = true;
        blockButton.disabled = true;
        throw error;
    });
    const commands = toolbarCommands(insertImage);

    for (const element of root.querySelectorAll<HTMLElement>("[data-shortcut]")) {
        const text = formatShortcut(element.dataset.shortcut ?? "");
        const label = element.querySelector(".shortcut");
        if (label) label.textContent = text;
        else element.title = `${element.getAttribute("aria-label")} (${text})`;
    }

    let current: HTMLButtonElement | null = null;

    const items = () => [...root.querySelectorAll<HTMLButtonElement>("button")].filter((item) => !inMenu(item) && !item.disabled && isVisible(item));

    const makeCurrent = (item: HTMLButtonElement | null) => {
        for (const button of root.querySelectorAll("button")) {
            if (!inMenu(button)) button.tabIndex = button === item ? 0 : -1;
        }
        current = item;
    };

    const sync = () => {
        for (const button of command) {
            const found = commands[button.dataset.command ?? ""];
            if (!found) continue;
            button.disabled = !found.run(view.state, undefined, view);
            if (found.active) button.setAttribute(inMenu(button) ? "aria-checked" : "aria-pressed", String(found.active(view.state)));
        }

        const type = currentBlockType(view.state);
        for (const option of blockOption) option.setAttribute("aria-checked", String(option.dataset.block === type));
        const known = blockLabel.some((label) => label.dataset.block === type);
        for (const label of blockLabel) label.hidden = label.dataset.block !== (known ? type : "mixed");

        blockButton.disabled = !canChangeBlockType(view.state);
        const panel = blockButton.popoverTargetElement as HTMLElement | null;
        if (blockButton.disabled && panel?.matches(":popover-open")) panel.hidePopover();

        if (!current || current.disabled || !isVisible(current)) makeCurrent(items()[0] ?? null);
    };

    onUpdate(sync, signal);

    return {
        runCommand(_event, button) {
            const name = button.dataset.command ?? "";
            commands[name]?.run(view.state, view.dispatch, view);
            if (name !== "link" && name !== "image") view.focus();
        },

        setBlock(_event, option) {
            BLOCKS[option.dataset.block ?? ""]?.(view.state, view.dispatch, view);
            view.focus();
        },

        track(event) {
            if (event.target instanceof HTMLButtonElement && !inMenu(event.target)) makeCurrent(event.target);
        },

        navigate(event) {
            if (event.defaultPrevented || (event.target instanceof Element && inMenu(event.target))) return;

            const list = items();
            const index = current ? list.indexOf(current) : -1;
            const next = {
                ArrowRight: list[(index + 1) % list.length],
                ArrowLeft: list[(index - 1 + list.length) % list.length],
                Home: list[0],
                End: list.at(-1),
            }[event.key];

            if (event.key === "Escape") {
                event.preventDefault();
                view.focus();
                return;
            }
            if (!next) return;
            event.preventDefault();
            makeCurrent(next);
            next.focus();
        },
    };
};
