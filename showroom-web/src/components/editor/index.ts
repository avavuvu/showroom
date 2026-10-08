import "prosemirror-view/style/prosemirror.css";
import "prosemirror-gapcursor/style/gapcursor.css";

import type { NewsletterEditor } from "@setups";
import { baseKeymap } from "prosemirror-commands";
import { dropCursor } from "prosemirror-dropcursor";
import { gapCursor } from "prosemirror-gapcursor";
import { history } from "prosemirror-history";
import { keymap } from "prosemirror-keymap";
import type { Node } from "prosemirror-model";
import { type Command, EditorState, NodeSelection, Selection } from "prosemirror-state";
import { EditorView } from "prosemirror-view";
import { buildKeymap, canInsert, toolbarCommands } from "./commands";
import { buildInputRules } from "./input-rules";
import { ImageView } from "./nodeviews/image";
import { placeholders, trailingParagraph, withTrailingParagraph } from "./plugins/basics";
import { linkPlugin, openLink } from "./plugins/link";
import { linkBarPlugin } from "./plugins/link-bar";
import { toolbarPlugin, formatShortcut } from "./plugins/toolbar";
import { imageFiles, uploadPlugin, Uploads } from "./plugins/upload";
import { SaveController, type SaveState } from "./save";
import { schema } from "./schema";

interface Props {
    id: string;
    revision: number;
    content: unknown;
}

const ALERT_STATES: SaveState[] = ["error", "conflict", "expired", "invalid"];

const messages = (saveKeys: string): Record<SaveState, string> => ({
    saved: "Saved",
    dirty: "Unsaved changes",
    saving: "Saving…",
    error: "Could not save. The editor will try again.",
    conflict: "This newsletter changed in another tab or window.",
    expired: `Your session ended. Log in again in a new tab, then press ${saveKeys}.`,
    invalid: "The server did not accept this content. Undo your last change, then try again.",
});

export const newsletterEditor: NewsletterEditor = (root, refs) => {
    const { signal } = refs;
    const props = JSON.parse(root.dataset.props ?? "{}") as Props;
    const text = messages(formatShortcut("Mod-s"));

    const setStatus = (state: string, message: string, alert: boolean) => {
        if (alert) refs.status.setAttribute("role", "alert");
        else refs.status.removeAttribute("role");
        refs.status.dataset.state = state;
        refs.status.textContent = message;
    };

    let doc: Node;
    try {
        doc = schema.nodeFromJSON(props.content);
        doc.check();
    } catch (error) {
        console.error("[editor] the stored document does not match the editor schema", error);
        setStatus("error", "This newsletter could not be loaded. Nothing was changed.", true);
        refs.title.disabled = true;
        refs.subtitle.disabled = true;
        for (const button of refs.command) button.disabled = true;
        refs.blockButton.disabled = true;
        refs.linkInput.disabled = true;
        return;
    }

    let problemTimer: ReturnType<typeof setTimeout> | undefined;
    const renderSaveState = (state: SaveState) => {
        clearTimeout(problemTimer);
        setStatus(state, text[state], ALERT_STATES.includes(state));
        refs.saveActions.hidden = state !== "conflict";
    };
    const showProblem = (message: string) => {
        setStatus("problem", message, true);
        clearTimeout(problemTimer);
        problemTimer = setTimeout(() => renderSaveState(saver.state), 8_000);
    };

    let pendingPick: ((files: File[]) => void) | null = null;
    refs.fileInput.addEventListener(
        "change",
        () => {
            const files = imageFiles(refs.fileInput.files);
            refs.fileInput.value = "";
            pendingPick?.(files);
            pendingPick = null;
        },
        { signal },
    );
    const pickFiles = (callback: (files: File[]) => void, multiple: boolean) => {
        pendingPick = callback;
        refs.fileInput.multiple = multiple;
        refs.fileInput.click();
    };

    const uploads = new Uploads(showProblem);

    const insertImage: Command = (state, dispatch, view) => {
        if (!canInsert(state, schema.nodes.figure)) return false;
        if (dispatch && view) pickFiles((files) => uploads.insert(view, files), true);
        return true;
    };

    const saver = new SaveController({
        url: `/json/${encodeURIComponent(props.id)}`,
        revision: props.revision,
        read: () => ({
            title: refs.title.value,
            subtitle: refs.subtitle.value.trim() || null,
            content: view.state.doc.toJSON(),
        }),
        onChange: renderSaveState,
        signal,
    });

    const commands = toolbarCommands({ openLink, insertImage });

    const view: EditorView = new EditorView(refs.surface, {
        state: EditorState.create({
            doc: withTrailingParagraph(doc),
            plugins: [
                buildInputRules(),
                keymap(buildKeymap(openLink)),
                keymap(baseKeymap),
                history(),
                dropCursor({ color: false, width: 2, class: "editor-drop-cursor" }),
                gapCursor(),
                placeholders({ body: "Write your newsletter…", caption: "Add a caption", attribution: "Author or source" }),
                trailingParagraph(),
                linkPlugin(),
                uploadPlugin(refs.uploadPlaceholder),
                toolbarPlugin(refs, commands, signal),
                linkBarPlugin(refs, { report: showProblem, signal }),
            ],
        }),
        attributes: {
            class: "flow",
            role: "textbox",
            "aria-multiline": "true",
            "aria-label": "Newsletter body",
            spellcheck: "true",
        },
        nodeViews: {
            image: (node, nodeView, getPos) => new ImageView(node, nodeView, getPos, { template: refs.imageFrame, pickFiles, uploads }),
        },
        handleClickOn(editor, _pos, node, nodePos, _event, direct) {
            if (!direct || node.type !== schema.nodes.image) return false;
            const figurePos = editor.state.doc.resolve(nodePos).before();
            editor.dispatch(editor.state.tr.setSelection(NodeSelection.create(editor.state.doc, figurePos)));
            return true;
        },
        handlePaste(editor, event) {
            const files = imageFiles(event.clipboardData?.files);
            if (files.length === 0) return false;
            uploads.insert(editor, files);
            return true;
        },
        handleDrop(editor, event, _slice, moved) {
            if (moved) return false;
            const files = imageFiles(event.dataTransfer?.files);
            if (files.length === 0) return false;
            event.preventDefault();
            uploads.insert(editor, files, editor.posAtCoords({ left: event.clientX, top: event.clientY })?.pos);
            return true;
        },
        dispatchTransaction(tr) {
            const { state, transactions } = view.state.applyTransaction(tr);
            view.updateState(state);
            if (transactions.some((transaction) => transaction.docChanged)) saver.markDirty();
        },
    });

    signal.addEventListener(
        "abort",
        () => {
            clearTimeout(problemTimer);
            view.destroy();
        },
        { once: true },
    );

    for (const input of [refs.title, refs.subtitle]) {
        input.addEventListener("input", () => saver.markDirty(), { signal });
    }
    refs.title.addEventListener(
        "keydown",
        (event) => {
            if (event.key !== "Enter") return;
            event.preventDefault();
            refs.subtitle.focus();
        },
        { signal },
    );
    refs.subtitle.addEventListener(
        "keydown",
        (event) => {
            if (event.key !== "Enter" && event.key !== "ArrowDown") return;
            event.preventDefault();
            view.dispatch(view.state.tr.setSelection(Selection.atStart(view.state.doc)));
            view.focus();
        },
        { signal },
    );

    window.addEventListener(
        "keydown",
        (event) => {
            if (!(event.metaKey || event.ctrlKey) || event.altKey || event.key.toLowerCase() !== "s") return;
            event.preventDefault();
            void saver.flush();
        },
        { signal },
    );

    window.addEventListener(
        "beforeunload",
        (event) => {
            if (!saver.unsaved && !uploads.active) return;
            event.preventDefault();
            event.returnValue = "";
        },
        { signal },
    );

    for (const link of refs.leave as HTMLElement[]) {
        link.addEventListener(
            "click",
            async (event) => {
                if (!(link instanceof HTMLAnchorElement) || !link.href) return;
                if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
                event.preventDefault();

                if (uploads.active) {
                    showProblem("Wait until the image upload is complete.");
                    return;
                }
                const saved = await saver.flush();
                if (saved || window.confirm("Your latest changes are not saved. Leave this page?")) {
                    saver.discard();
                    window.location.assign(link.href);
                }
            },
            { signal },
        );
    }

    refs.reload.addEventListener(
        "click",
        () => {
            saver.discard();
            window.location.reload();
        },
        { signal },
    );
    refs.overwrite.addEventListener("click", () => void saver.overwrite(), { signal });
};
