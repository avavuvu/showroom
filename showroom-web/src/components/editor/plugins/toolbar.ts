import { Plugin } from "prosemirror-state";
import type { EditorView } from "prosemirror-view";
import { blockTypeCommand, canChangeBlockType, currentBlockType, type ToolbarCommand } from "../commands";
import { PopupMenu } from "../menu";

interface ToolbarRefs {
    toolbar: HTMLElement;
    command: HTMLButtonElement[];
    blockButton: HTMLButtonElement;
    blockCurrent: HTMLElement;
    blockOptions: HTMLElement;
    blockOption: HTMLButtonElement[];
}

const OTHER_LABELS: Record<string, string> = {
    caption: "Caption",
    attribution: "Source",
    mixed: "Mixed",
};

const isMac = /Mac|iPhone|iPad/.test(navigator.platform);

export function formatShortcut(shortcut: string): string {
    const parts = shortcut.split("-");
    const key = parts.pop()?.toUpperCase() ?? "";
    const has = (name: string) => parts.includes(name);

    if (isMac) {
        return [has("Ctrl") ? "⌃" : "", has("Alt") ? "⌥" : "", has("Shift") ? "⇧" : "", has("Mod") ? "⌘" : "", key].join("");
    }
    return [has("Mod") || has("Ctrl") ? "Ctrl" : "", has("Alt") ? "Alt" : "", has("Shift") ? "Shift" : "", key]
        .filter(Boolean)
        .join("+");
}

class BlockMenu {
    private current = "paragraph";
    private menu: PopupMenu;

    constructor(
        private view: EditorView,
        private refs: ToolbarRefs,
        signal: AbortSignal,
    ) {
        for (const option of refs.blockOption) {
            const shortcut = option.dataset.shortcut;
            const label = option.querySelector(".shortcut");
            if (shortcut && label) label.textContent = formatShortcut(shortcut);
        }

        this.menu = new PopupMenu({
            button: refs.blockButton,
            panel: refs.blockOptions,
            items: refs.blockOption,
            current: () => refs.blockOption.find((option) => option.dataset.block === this.current),
            onChoose: (option) => {
                blockTypeCommand(option.dataset.block ?? "")?.(this.view.state, this.view.dispatch, this.view);
                this.view.focus();
            },
            onEscape: () => this.view.focus(),
            signal,
        });
    }

    sync(view: EditorView) {
        this.view = view;
        const { state } = view;
        this.current = currentBlockType(state);
        const { blockButton, blockCurrent, blockOption } = this.refs;

        let selected: HTMLButtonElement | undefined;
        for (const option of blockOption) {
            const checked = option.dataset.block === this.current;
            option.setAttribute("aria-checked", String(checked));
            if (checked) selected = option;
        }

        const icon = selected?.querySelector("svg")?.cloneNode(true);
        const label = document.createElement("span");
        label.className = "option-label";
        label.textContent = selected?.querySelector(".option-label")?.textContent ?? OTHER_LABELS[this.current] ?? "Mixed";
        blockCurrent.replaceChildren(...(icon ? [icon, label] : [label]));

        blockButton.disabled = !canChangeBlockType(state);
        if (blockButton.disabled) this.menu.close(false);
    }
}

class Toolbar {
    private current: HTMLElement | null = null;
    private menu: BlockMenu;

    constructor(
        private view: EditorView,
        private refs: ToolbarRefs,
        private commands: Record<string, ToolbarCommand>,
        signal: AbortSignal,
    ) {
        for (const button of refs.command) {
            const name = button.dataset.command ?? "";
            const command = commands[name];
            if (!command) {
                console.warn(`[editor] there is no toolbar command named "${name}"`);
                button.hidden = true;
                continue;
            }

            const label = button.getAttribute("aria-label") ?? name;
            if (button.dataset.shortcut) button.title = `${label} (${formatShortcut(button.dataset.shortcut)})`;

            button.addEventListener("mousedown", (event) => event.preventDefault(), { signal });
            button.addEventListener(
                "click",
                () => {
                    const { state, dispatch } = this.view;
                    command.run(state, dispatch, this.view);
                    if (name !== "link" && name !== "image") this.view.focus();
                },
                { signal },
            );
        }

        this.menu = new BlockMenu(view, refs, signal);

        refs.toolbar.addEventListener("keydown", (event) => this.navigate(event), { signal });
        refs.toolbar.addEventListener(
            "focusin",
            (event) => {
                if (event.target instanceof HTMLElement && this.items().includes(event.target)) this.makeCurrent(event.target);
            },
            { signal },
        );

        this.sync(view);
    }

    sync(view: EditorView) {
        this.view = view;
        const { state } = view;

        for (const button of this.refs.command) {
            const command = this.commands[button.dataset.command ?? ""];
            if (!command) continue;
            button.disabled = !command.run(state, undefined, view);
            if (command.active) button.setAttribute("aria-pressed", String(command.active(state)));
        }

        this.menu.sync(view);

        const items = this.items();
        if (!this.current || !items.includes(this.current)) this.makeCurrent(items[0] ?? null);
    }

    private items(): HTMLElement[] {
        return [...this.refs.toolbar.querySelectorAll<HTMLElement>("button, input, a")].filter(
            (item) =>
                !item.closest('[role="menu"]') &&
                !item.hidden &&
                !(item as HTMLButtonElement | HTMLInputElement).disabled &&
                item.getAttribute("aria-disabled") !== "true",
        );
    }

    private makeCurrent(item: HTMLElement | null) {
        for (const element of this.refs.toolbar.querySelectorAll<HTMLElement>("button, input, a")) {
            if (element.closest('[role="menu"]')) continue;
            element.tabIndex = element === item ? 0 : -1;
        }
        this.current = item;
    }

    private navigate(event: KeyboardEvent) {
        if (event.defaultPrevented || event.target instanceof HTMLInputElement) return;
        if ((event.target as Element | null)?.closest('[role="menu"]')) return;

        const items = this.items();
        const index = this.current ? items.indexOf(this.current) : -1;
        let next: HTMLElement | undefined;

        switch (event.key) {
            case "ArrowRight":
                next = items[(index + 1) % items.length];
                break;
            case "ArrowLeft":
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
                this.view.focus();
                return;
            default:
                return;
        }

        if (!next) return;
        event.preventDefault();
        this.makeCurrent(next);
        next.focus();
    }
}

export function toolbarPlugin(refs: ToolbarRefs, commands: Record<string, ToolbarCommand>, signal: AbortSignal) {
    return new Plugin({
        view(view) {
            const toolbar = new Toolbar(view, refs, commands, signal);
            return { update: (next) => toolbar.sync(next) };
        },
    });
}
