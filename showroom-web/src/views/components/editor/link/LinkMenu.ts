import { Extension, getMarkRange, type Editor } from "@tiptap/core";
import type { Mark, MarkType, Node as PMNode } from "@tiptap/pm/model";
import { Plugin, PluginKey } from "@tiptap/pm/state";
import { Decoration, DecorationSet } from "@tiptap/pm/view";
import { createApp, type App } from "vue";
import LinkMenu from "./LinkMenu.vue";
import "./link-menu.css";

interface LinkRange {
    from: number;
    to: number;
}

interface LinkMenuState {
    range: LinkRange | null;
    href: string;
    autofocus: boolean;
    sessionId: number;
}

export const linkMenuPluginKey = new PluginKey<LinkMenuState>("linkMenu");

const CONTAINER_CLASS = "link-menu-container";

let nextSessionId = 0;

export function openLinkMenu(
    editor: Editor,
    range: LinkRange,
    href: string,
    autofocus: boolean,
) {
    editor.view.dispatch(
        editor.state.tr.setMeta(linkMenuPluginKey, {
            type: "open",
            range,
            href,
            autofocus,
            sessionId: ++nextSessionId,
        }),
    );
}

export function closeLinkMenu(editor: Editor) {
    editor.view.dispatch(
        editor.state.tr
            .setStoredMarks(null)
            .setMeta(linkMenuPluginKey, { type: "close" }),
    );
}

export function triggerLinkMenu(editor: Editor) {
    const linkType = editor.state.schema.marks.link;
    if (!linkType) return;

    const { $from, empty, from, to } = editor.state.selection;
    const existingRange = getMarkRange($from, linkType);

    if (existingRange) {
        openLinkMenu(editor, existingRange, editor.getAttributes("link").href ?? "", true);
        return;
    }

    if (!empty) {
        editor.chain().focus().setLink({ href: "" }).run();
        openLinkMenu(editor, { from, to }, "", true);
        return;
    }

    editor.chain().focus().setTextSelection(from).run();
    openLinkMenu(editor, { from, to: from }, "", false);
}

function closeAndFocus(editor: Editor, pos: number) {
    editor.chain().focus().setTextSelection(pos).run();
    closeLinkMenu(editor);
}

function currentRange(editor: Editor): LinkRange | null {
    return linkMenuPluginKey.getState(editor.state)?.range ?? null;
}

function findLinkMark(
    doc: PMNode,
    range: LinkRange,
    linkType: MarkType,
): Mark | null {
    let found: Mark | null = null;
    doc.nodesBetween(range.from, range.to, (node) => {
        if (found) return false;
        const mark = node.marks.find((m) => m.type === linkType);
        if (mark) found = mark;
        return true;
    });
    return found;
}

function positionLinkMenu(
    editor: Editor,
    container: HTMLElement,
    range: LinkRange,
    blockPos: number,
) {
    requestAnimationFrame(() => {
        try {
            const view = editor.view;
            const linkCoords = view.coordsAtPos(range.from);
            const blockRect = (view.nodeDOM(blockPos) as HTMLElement | null)
                ?.getBoundingClientRect();
            if (!blockRect) return;

            container.style.left = `${blockRect.left}px`;
            container.style.top = `${linkCoords.bottom}px`;
            container.style.width = `${blockRect.width}px`;
            container.style.setProperty("--left", `${Math.max(0, linkCoords.left - blockRect.left)}px`);
        } catch {}
    });
}

function buildLinkMenuDom(
    editor: Editor,
    href: string,
    autofocus: boolean,
    range: LinkRange,
    blockPos: number,
): HTMLElement {
    const container = document.createElement("div");
    container.contentEditable = "false";
    container.className = CONTAINER_CLASS;

    const app: App = createApp(LinkMenu, {
        initialHref: href,
        autofocus,
        onSave: (newHref: string) => {
            const current = currentRange(editor);
            if (!current) return;
            editor
                .chain()
                .focus()
                .setTextSelection(current)
                .extendMarkRange("link")
                .setLink({ href: newHref })
                .run();
            closeAndFocus(editor, current.to);
        },
        onRemove: () => {
            const current = currentRange(editor);
            if (!current) return;
            editor
                .chain()
                .focus()
                .setTextSelection(current)
                .extendMarkRange("link")
                .unsetLink()
                .run();
            closeAndFocus(editor, current.from);
        },
        onClose: () => {
            const current = currentRange(editor);
            closeAndFocus(editor, current?.to ?? range.to);
        },
    });
    app.mount(container);
    (container as HTMLElement & { __vueApp?: App }).__vueApp = app;

    positionLinkMenu(editor, container, range, blockPos);

    return container;
}

export const LinkMenuExtension = Extension.create({
    name: "linkMenu",

    addStorage() {
        return {
            outsideClickHandler: null as ((event: MouseEvent) => void) | null,
        };
    },

    onCreate() {
        const editor = this.editor;

        const handler = (event: MouseEvent) => {
            const target = event.target as Node;
            if (editor.view.dom.contains(target)) return;
            if ((target as HTMLElement).closest?.(`.${CONTAINER_CLASS}`)) return;
            if (!currentRange(editor)) return;
            closeLinkMenu(editor);
        };

        document.addEventListener("mousedown", handler);
        this.storage.outsideClickHandler = handler;
    },

    onDestroy() {
        if (this.storage.outsideClickHandler) {
            document.removeEventListener("mousedown", this.storage.outsideClickHandler);
        }
    },

    addKeyboardShortcuts() {
        return {
            "Mod-k": () => {
                triggerLinkMenu(this.editor);
                return true;
            },
        };
    },

    addProseMirrorPlugins() {
        const editor = this.editor;

        return [
            new Plugin<LinkMenuState>({
                key: linkMenuPluginKey,
                state: {
                    init: () => ({
                        range: null,
                        href: "",
                        autofocus: false,
                        sessionId: 0,
                    }),
                    apply(tr, prev) {
                        const meta = tr.getMeta(linkMenuPluginKey);
                        if (meta?.type === "open") {
                            return {
                                range: meta.range,
                                href: meta.href,
                                autofocus: meta.autofocus,
                                sessionId: meta.sessionId,
                            };
                        }
                        if (meta?.type === "close") {
                            return { ...prev, range: null };
                        }

                        if (prev.range) {
                            const from = tr.mapping.map(prev.range.from, -1);
                            const to = tr.mapping.map(prev.range.to, 1);
                            if (from > to) return { ...prev, range: null };
                            return { ...prev, range: { from, to } };
                        }

                        return prev;
                    },
                },
                appendTransaction(_transactions, _oldState, newState) {
                    const pluginState = linkMenuPluginKey.getState(newState);
                    const range = pluginState?.range;
                    if (!range) return null;

                    const linkType = newState.schema.marks.link;
                    if (!linkType) return null;

                    const { selection } = newState;
                    if (!selection.empty) return null;
                    if (selection.from < range.from || selection.from > range.to) {
                        return null;
                    }

                    const alreadyActive = selection.$from
                        .marks()
                        .some((m) => m.type === linkType);
                    if (alreadyActive) return null;

                    const linkMark =
                        findLinkMark(newState.doc, range, linkType) ??
                        linkType.create({ href: pluginState.href });

                    return newState.tr
                        .setStoredMarks([linkMark])
                        .setMeta("addToHistory", false);
                },
                props: {
                    handleClick(view, pos) {
                        const linkType = view.state.schema.marks.link;
                        if (!linkType) return false;

                        const $pos = view.state.doc.resolve(pos);
                        const range = getMarkRange($pos, linkType);

                        if (range) {
                            const href =
                                findLinkMark(view.state.doc, range, linkType)?.attrs.href ?? "";
                            openLinkMenu(editor, range, href, false);
                        } else {
                            closeLinkMenu(editor);
                        }

                        return false;
                    },
                    handleKeyDown(view, event) {
                        const range = linkMenuPluginKey.getState(view.state)?.range;
                        if (!range) return false;

                        if (event.key === "Escape" || event.key === "Enter") {
                            closeLinkMenu(editor);
                            return true;
                        }

                        if (event.key !== "ArrowLeft" && event.key !== "ArrowRight") {
                            return false;
                        }

                        const { from, to } = view.state.selection;
                        if (event.key === "ArrowLeft" && from <= range.from) return true;
                        if (event.key === "ArrowRight" && to >= range.to) return true;
                        return false;
                    },
                    decorations(state) {
                        const pluginState = linkMenuPluginKey.getState(state);
                        const range = pluginState?.range;
                        if (!range || !pluginState) return DecorationSet.empty;

                        const activeLink = Decoration.inline(range.from, range.to, {
                            class: "is-active-link",
                        });

                        const $from = state.doc.resolve(range.from);
                        const blockPos = $from.before($from.depth);

                        const widget = Decoration.widget(
                            range.to,
                            () =>
                                buildLinkMenuDom(
                                    editor,
                                    pluginState.href,
                                    pluginState.autofocus,
                                    range,
                                    blockPos,
                                ),
                            {
                                side: 1,
                                stopEvent: () => true,
                                key: `link-menu-${pluginState.sessionId}`,
                                destroy: (node) => {
                                    const app = (node as HTMLElement & { __vueApp?: App })
                                        .__vueApp;
                                    app?.unmount();
                                },
                            },
                        );

                        return DecorationSet.create(state.doc, [activeLink, widget]);
                    },
                },
            }),
        ];
    },
});
