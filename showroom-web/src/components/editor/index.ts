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
import { trackViewport } from "./viewport";

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
        setStatus("error", "This newsletter could not be loaded. Nothing was changed.", true);
        refs.title.disabled = true;
        refs.subtitle.disabled = true;
        for (const button of refs.command) button.disabled = true;
        refs.blockButton.disabled = true;
        refs.linkInput.disabled = true;
        throw new Error("the stored document does not match the editor schema", { cause: error });
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

    const toolbar = refs.toolbar.closest<HTMLElement>(".editor-toolbar") ?? refs.toolbar;
    const stopViewport = trackViewport(root, root.querySelector<HTMLElement>(":scope > header"), toolbar, view, signal);

    return {
        filesPicked() {
            const files = imageFiles(refs.fileInput.files);
            refs.fileInput.value = "";
            pendingPick?.(files);
            pendingPick = null;
        },

        markDirty: () => saver.markDirty(),

        toSubtitle: () => refs.subtitle.focus(),

        toBody() {
            view.dispatch(view.state.tr.setSelection(Selection.atStart(view.state.doc)));
            view.focus();
        },

        save: () => void saver.flush(),

        guard(event) {
            if (!saver.unsaved && !uploads.active) return;
            event.preventDefault();
            event.returnValue = "";
        },

        async leave(event, link) {
            if (!(link instanceof HTMLAnchorElement) || !link.href || event.defaultPrevented) return;
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

        reload() {
            saver.discard();
            window.location.reload();
        },

        overwrite: () => void saver.overwrite(),

        disconnect() {
            clearTimeout(problemTimer);
            stopViewport();
            saver.stop();
            view.destroy();
        },
    };
};
