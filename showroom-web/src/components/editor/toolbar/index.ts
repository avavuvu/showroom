import { inject } from "@bq/components/src/setups";
import type { EditorToolbar } from "@setups";
import { editorContext } from "../context";
import { blockTypeCommand, canChangeBlockType, currentBlockType } from "../document/commands";
import { openLink } from "../link_bar/link";
import { formatShortcut } from "../shortcuts";
import { toolbarCommands } from "./commands";

const OTHER_LABELS: Record<string, string> = {
    caption: "Caption",
    attribution: "Source",
    mixed: "Mixed",
};

const isVisible = (element: HTMLElement) => element.getClientRects().length > 0;
const inMenu = (element: Element) => element.closest('[role="menu"]') !== null;

export const editorToolbar: EditorToolbar = async (root, { command, blockButton, blockCurrent, blockOption }, { signal }) => {
    const { view, onUpdate, insertImage } = await inject(root, editorContext).catch((error: unknown) => {
        for (const button of command) button.disabled = true;
        blockButton.disabled = true;
        throw error;
    });
    const commands = toolbarCommands({ openLink, insertImage });

    for (const option of blockOption) {
        const label = option.querySelector(".shortcut");
        if (label && option.dataset.shortcut) label.textContent = formatShortcut(option.dataset.shortcut);
    }

    for (const button of command) {
        const name = button.dataset.command ?? "";
        const found = commands[name];
        if (!found) {
            console.warn(`[editor] there is no toolbar command named "${name}"`);
            button.hidden = true;
            continue;
        }
        if (inMenu(button)) {
            button.setAttribute("role", found.active ? "menuitemcheckbox" : "menuitem");
            const shortcut = button.querySelector(".shortcut");
            if (shortcut && button.dataset.shortcut) shortcut.textContent = formatShortcut(button.dataset.shortcut);
        } else if (button.dataset.shortcut) {
            button.title = `${button.getAttribute("aria-label") ?? name} (${formatShortcut(button.dataset.shortcut)})`;
        }
    }

    let current: HTMLElement | null = null;

    const items = (): HTMLElement[] =>
        [...root.querySelectorAll<HTMLElement>("button, input, a")].filter(
            (item) =>
                !inMenu(item) &&
                isVisible(item) &&
                !item.hidden &&
                !(item as HTMLButtonElement | HTMLInputElement).disabled &&
                item.getAttribute("aria-disabled") !== "true",
        );

    const makeCurrent = (item: HTMLElement | null) => {
        for (const element of root.querySelectorAll<HTMLElement>("button, input, a")) {
            if (!inMenu(element)) element.tabIndex = element === item ? 0 : -1;
        }
        current = item;
    };

    const syncBlock = () => {
        const type = currentBlockType(view.state);
        let selected: HTMLButtonElement | undefined;
        for (const option of blockOption) {
            const checked = option.dataset.block === type;
            option.setAttribute("aria-checked", String(checked));
            if (checked) selected = option;
        }

        const icon = selected?.querySelector("svg")?.cloneNode(true);
        const label = document.createElement("span");
        label.className = "option-label";
        label.textContent = selected?.querySelector(".option-label")?.textContent ?? OTHER_LABELS[type] ?? "Mixed";
        blockCurrent.replaceChildren(...(icon ? [icon, label] : [label]));
        blockButton.disabled = !canChangeBlockType(view.state);
    };

    const sync = () => {
        for (const button of command) {
            const found = commands[button.dataset.command ?? ""];
            if (!found) continue;
            button.disabled = !found.run(view.state, undefined, view);
            if (found.active) button.setAttribute(inMenu(button) ? "aria-checked" : "aria-pressed", String(found.active(view.state)));
        }
        syncBlock();

        const list = items();
        if (!current || !list.includes(current)) makeCurrent(list[0] ?? null);
    };

    onUpdate(sync, signal);

    return {
        runCommand(_event, button) {
            const name = button.dataset.command ?? "";
            commands[name]?.run(view.state, view.dispatch, view);
            if (name !== "link" && name !== "image") view.focus();
        },

        setBlock(_event, option) {
            blockTypeCommand(option.dataset.block ?? "")?.(view.state, view.dispatch, view);
            view.focus();
        },

        track(event) {
            if (event.target instanceof HTMLElement && items().includes(event.target)) makeCurrent(event.target);
        },

        navigate(event) {
            if (event.defaultPrevented || event.target instanceof HTMLInputElement) return;
            if (event.target instanceof Element && inMenu(event.target)) return;

            const list = items();
            const index = current ? list.indexOf(current) : -1;
            let next: HTMLElement | undefined;

            switch (event.key) {
                case "ArrowRight":
                    next = list[(index + 1) % list.length];
                    break;
                case "ArrowLeft":
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
                    view.focus();
                    return;
                default:
                    return;
            }

            if (!next) return;
            event.preventDefault();
            makeCurrent(next);
            next.focus();
        },
    };
};
