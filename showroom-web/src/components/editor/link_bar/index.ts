import { inject } from "@bq/components/src/setups";
import type { EditorLinkBar } from "@setups";
import { editorContext } from "../context";
import { applyLink, closeLink, linkEditing, linkText, openLink, removeLink, selectedLink } from "./link";

export const editorLinkBar: EditorLinkBar = async (root, { text, url, open, remove }, { signal }) => {
    const { view, onUpdate } = await inject(root, editorContext).catch((error: unknown) => {
        text.disabled = true;
        url.disabled = true;
        throw error;
    });

    let wasEditing = false;

    const sync = () => {
        const { state } = view;
        const editing = linkEditing(state);
        const range = selectedLink(state);
        const target = editing ?? range;
        const href = editing?.href || (range?.mark.attrs.href as string | undefined) || "";
        const focused = root.contains(document.activeElement);

        root.classList.toggle("is-idle", !target);
        if (href) open.href = href;
        else open.removeAttribute("href");
        remove.disabled = !href;

        if (!focused) {
            text.value = target ? linkText(state, target) : "";
            url.value = href;
            url.setCustomValidity("");
        }

        if (editing && !wasEditing && !focused) {
            const first = editing.from === editing.to ? text : url;
            requestAnimationFrame(() => {
                first.focus();
                first.select();
            });
        }
        wasEditing = !!editing;
    };

    onUpdate(sync, signal);

    return {
        start() {
            if (!linkEditing(view.state)) openLink(view.state, view.dispatch, view);
        },

        leave() {
            requestAnimationFrame(() => {
                if (view.isDestroyed || root.contains(document.activeElement)) return;
                closeLink(view);
                sync();
            });
        },

        cancel() {
            closeLink(view);
            view.focus();
        },

        apply() {
            const problem = applyLink(view, url.value, text.value);
            if (!problem) {
                view.focus();
                return;
            }
            url.setCustomValidity(problem);
            url.reportValidity();
        },

        clearProblem: () => url.setCustomValidity(""),

        unlink() {
            removeLink(view);
            view.focus();
        },
    };
};
