import type { NewsletterEditor } from "@setups";
import { baseKeymap } from "prosemirror-commands";
import { dropCursor } from "prosemirror-dropcursor";
import { gapCursor } from "prosemirror-gapcursor";
import { history } from "prosemirror-history";
import { keymap } from "prosemirror-keymap";
import type { Node } from "prosemirror-model";
import { EditorState, NodeSelection } from "prosemirror-state";
import { EditorView } from "prosemirror-view";
import { placeholders, trailingParagraph, withTrailingParagraph } from "./prosemirror/basics";

import { buildInputRules } from "./prosemirror/input-rules";
import { buildKeymap } from "./prosemirror/keymap";
import { schema } from "./prosemirror/schema";
import type { PickFiles } from "./images/file-picker";
import { ImageView } from "./images/image-view";
import { imageFiles, uploadPlugin, type Uploads } from "./images/upload";
import { linkPlugin } from "./link_bar/link";

type Refs = Parameters<NewsletterEditor>[1];

interface ViewOptions {
    doc: Node;
    uploads: Uploads;
    pickFiles: PickFiles;
    onChange: () => void;
    onUpdate: () => void;
}

export function createView(refs: Refs, { doc, uploads, pickFiles, onChange, onUpdate }: ViewOptions): EditorView {
    const view: EditorView = new EditorView(refs.surface, {
        state: EditorState.create({
            doc: withTrailingParagraph(doc),
            plugins: [
                buildInputRules(),
                keymap(buildKeymap()),
                keymap(baseKeymap),
                history(),
                dropCursor({ color: false, width: 2, class: "editor-drop-cursor" }),
                gapCursor(),
                placeholders({ body: "Write your newsletter…", caption: "Add a caption", attribution: "Author or source" }),
                trailingParagraph(),
                linkPlugin(),
                uploadPlugin(refs.uploadPlaceholder),
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
            onUpdate();
            if (transactions.some((transaction) => transaction.docChanged)) onChange();
        },
    });

    return view;
}
