import type { NewsletterEditor } from "@setups"
import { baseKeymap } from "prosemirror-commands"
import { dropCursor } from "prosemirror-dropcursor"
import { gapCursor } from "prosemirror-gapcursor"
import { history } from "prosemirror-history"
import { keymap } from "prosemirror-keymap"
import type { Node } from "prosemirror-model"
import { EditorState, NodeSelection } from "prosemirror-state"
import { EditorView } from "prosemirror-view"
import { ensureTrailingParagraph, placeholders, trailingParagraph } from "./prosemirror/basics"

import { buildInputRules } from "./prosemirror/input-rules"
import { buildKeymap } from "./prosemirror/keymap"
import { namePlugin } from "./prosemirror/name"
import { schema } from "./prosemirror/schema"
import { imageView } from "../../blocks/image/editor/view"
import { filterImageFiles, type Uploads } from "../../blocks/image/upload"
import { musicView } from "../../blocks/music/editor/view"
import { musicPlugin } from "../../blocks/music/paste"
import type { PickFiles } from "./file-picker"
import { linkPlugin } from "./link_bar/link"

type Refs = Parameters<NewsletterEditor>[1]

interface ViewOptions {
    doc: Node
    uploads: Uploads
    pickFiles: PickFiles
    onChange: () => void
    onUpdate: () => void
}

export const createView = (refs: Refs, { doc, uploads, pickFiles, onChange, onUpdate }: ViewOptions): EditorView => {
    const view: EditorView = new EditorView(refs.surface, {
        state: EditorState.create({
            doc: ensureTrailingParagraph(doc),
            plugins: [
                buildInputRules(),
                keymap(buildKeymap()),
                keymap(baseKeymap),
                history(),
                dropCursor({ color: false, width: 2, class: "editor-drop-cursor" }),
                gapCursor(),
                placeholders({ body: "Write your newsletter…", caption: "Add a caption", attribution: "Author or source" }),
                trailingParagraph(),
                namePlugin(),
                musicPlugin(refs.musicLookup),
                linkPlugin(),
                uploads.plugin(),
            ],
        }),
        attributes: {
            class: "flow",
            role: "textbox",
            "aria-multiline": "true",
            "aria-label": "Newsletter body",
            spellcheck: "true",
            autocapitalize: "sentences",
            autocorrect: "on",
            enterkeyhint: "enter",
        },
        nodeViews: {
            image: imageView({ template: refs.imageFrame, pickFiles, uploads }),
            music: musicView({ template: refs.musicFrame, pickFiles, uploads }),
        },
        handleClickOn: (editor, _pos, node, nodePos, _event, isDirect) => {
            if (!isDirect || node.type !== schema.nodes.image) {
                return false
            }
            const figurePos = editor.state.doc.resolve(nodePos).before()
            editor.dispatch(editor.state.tr.setSelection(NodeSelection.create(editor.state.doc, figurePos)))
            return true
        },
        handlePaste: (editor, event) => {
            const files = filterImageFiles(event.clipboardData?.files)
            if (files.length === 0) {
                return false
            }
            uploads.insert(editor, files)
            return true
        },
        handleDrop: (editor, event, _slice, wasMoved) => {
            if (wasMoved) {
                return false
            }
            const files = filterImageFiles(event.dataTransfer?.files)
            if (files.length === 0) {
                return false
            }
            event.preventDefault()
            uploads.insert(editor, files, editor.posAtCoords({ left: event.clientX, top: event.clientY })?.pos)
            return true
        },
        dispatchTransaction: (transaction) => {
            const { state, transactions } = view.state.applyTransaction(transaction)
            view.updateState(state)
            onUpdate()
            if (transactions.some((applied) => applied.docChanged)) {
                onChange()
            }
        },
    })

    return view
}
