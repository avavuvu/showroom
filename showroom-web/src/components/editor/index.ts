import "prosemirror-view/style/prosemirror.css";
import "prosemirror-gapcursor/style/gapcursor.css";

import { provide } from "../../../bindings/boutique/setups";
import type { NewsletterEditor } from "@setups";
import type { Node } from "prosemirror-model";
import { Selection } from "prosemirror-state";
import type { EditorView } from "prosemirror-view";
import { editorContext } from "./context";
import { schema } from "./prosemirror/schema";
import { filePicker } from "./images/file-picker";
import { insertImage, Uploads } from "./images/upload";
import { SaveController } from "./save";
import { SaveStatus } from "./save_status/status";
import { createView } from "./view";
import { trackViewport } from "./viewport";

interface Props {
    id: string;
    revision: number;
    content: unknown;
}

export const newsletterEditor: NewsletterEditor = (root, refs, { signal }) => {
    const props = JSON.parse(root.dataset.props ?? "{}") as Props;
    const status = new SaveStatus(refs.status, refs.saveActions);

    let doc: Node;
    try {
        doc = schema.nodeFromJSON(props.content);
        doc.check();
    } catch (error) {
        status.set("error", "This newsletter could not be loaded. Nothing was changed.", true);
        refs.title.disabled = true;
        refs.subtitle.disabled = true;
        throw new Error("the stored document does not match the editor schema", { cause: error });
    }

    const report = (message: string) => status.problem(message, () => saver.state);
    const files = filePicker(refs.fileInput);
    const uploads = new Uploads(report);

    const saver = new SaveController({
        url: `/json/${encodeURIComponent(props.id)}`,
        revision: props.revision,
        read: () => ({
            title: refs.title.value,
            subtitle: refs.subtitle.value.trim() || null,
            content: view.state.doc.toJSON(),
        }),
        onChange: (state) => status.render(state),
        signal,
    });

    const listeners = new Set<(view: EditorView) => void>();
    const view = createView(refs, {
        doc,
        uploads,
        pickFiles: files.pick,
        onChange: () => saver.markDirty(),
        onUpdate: () => listeners.forEach((listener) => listener(view)),
    });

    provide(root, editorContext, {
        view,
        insertImage: insertImage(uploads, files.pick),
        onUpdate(listener, signal) {
            listeners.add(listener);
            signal.addEventListener("abort", () => listeners.delete(listener), { once: true });
            listener(view);
        },
    });

    const stopViewport = trackViewport(root, refs.header, refs.toolbar, view, signal);

    return {
        filesPicked: () => files.picked(),

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
                report("Wait until the image upload is complete.");
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
            status.stop();
            stopViewport();
            saver.stop();
            view.destroy();
        },
    };
};
