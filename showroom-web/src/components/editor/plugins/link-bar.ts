import { Plugin } from "prosemirror-state";
import type { EditorView } from "prosemirror-view";
import { applyLink, closeLink, linkEditing, linkText, openLink, removeLink, selectedLink } from "./link";

interface LinkBarRefs {
    linkBar: HTMLElement;
    linkText: HTMLInputElement;
    linkInput: HTMLInputElement;
    linkOpen: HTMLAnchorElement;
    linkRemove: HTMLButtonElement;
}

interface LinkBarOptions {
    report: (message: string) => void;
    signal: AbortSignal;
}

class LinkBar {
    private wasEditing = false;

    constructor(
        private view: EditorView,
        private refs: LinkBarRefs,
        options: LinkBarOptions,
    ) {
        const { signal } = options;
        const { linkBar, linkText: textInput, linkInput: urlInput, linkOpen, linkRemove } = refs;

        for (const input of [textInput, urlInput]) {
            input.addEventListener(
                "focus",
                () => {
                    if (!linkEditing(this.view.state)) openLink(this.view.state, this.view.dispatch, this.view);
                },
                { signal },
            );
            input.addEventListener(
                "blur",
                () =>
                    requestAnimationFrame(() => {
                        if (this.view.isDestroyed || linkBar.contains(document.activeElement)) return;
                        closeLink(this.view);
                        this.sync();
                    }),
                { signal },
            );
            input.addEventListener(
                "keydown",
                (event) => {
                    if (event.key === "Escape") {
                        event.preventDefault();
                        closeLink(this.view);
                        this.view.focus();
                        return;
                    }
                    if (event.key !== "Enter") return;
                    event.preventDefault();
                    const problem = applyLink(this.view, urlInput.value, textInput.value);
                    if (problem) {
                        urlInput.setAttribute("aria-invalid", "true");
                        urlInput.focus();
                        options.report(problem);
                        return;
                    }
                    this.view.focus();
                },
                { signal },
            );
        }
        urlInput.addEventListener("input", () => urlInput.removeAttribute("aria-invalid"), { signal });

        linkRemove.addEventListener("mousedown", (event) => event.preventDefault(), { signal });
        linkRemove.addEventListener(
            "click",
            () => {
                removeLink(this.view);
                this.view.focus();
            },
            { signal },
        );
        linkOpen.addEventListener(
            "click",
            (event) => {
                if (linkOpen.getAttribute("aria-disabled") === "true") event.preventDefault();
            },
            { signal },
        );

        this.sync();
    }

    update(view: EditorView) {
        this.view = view;
        this.sync();
    }

    private sync() {
        const { linkBar, linkText: textInput, linkInput: urlInput, linkOpen, linkRemove } = this.refs;
        const { state } = this.view;
        const editing = linkEditing(state);
        const range = selectedLink(state);
        const target = editing ?? range;
        const href = editing?.href || (range?.mark.attrs.href as string | undefined) || "";

        linkBar.classList.toggle("is-idle", !target);
        linkOpen.href = href || "#";
        linkOpen.setAttribute("aria-disabled", String(!href));
        linkRemove.disabled = !href;

        if (!linkBar.contains(document.activeElement)) {
            textInput.value = target ? linkText(state, target) : "";
            urlInput.value = href;
            urlInput.removeAttribute("aria-invalid");
        }

        if (editing && !this.wasEditing && !linkBar.contains(document.activeElement)) {
            const first = editing.from === editing.to ? textInput : urlInput;
            requestAnimationFrame(() => {
                first.focus();
                first.select();
            });
        }
        this.wasEditing = !!editing;
    }
}

export function linkBarPlugin(refs: LinkBarRefs, options: LinkBarOptions) {
    return new Plugin({
        view: (view) => new LinkBar(view, refs, options),
    });
}
