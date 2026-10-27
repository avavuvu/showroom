import type { NewsletterEditor } from "@setups"
import { baseKeymap } from "prosemirror-commands"
import { dropCursor } from "prosemirror-dropcursor"
import { gapCursor } from "prosemirror-gapcursor"
import { history } from "prosemirror-history"
import { keymap } from "prosemirror-keymap"
import type { Node } from "prosemirror-model"
import { EditorState, NodeSelection, type Plugin } from "prosemirror-state"
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
import { youtubeView } from "../../blocks/youtube/editor/view"
import { youtubePlugin } from "../../blocks/youtube/paste"
import { boxoutPlugin } from "../../blocks/boxout/commands"
import { boxoutView } from "../../blocks/boxout/editor/view"
import type { PickFiles } from "./file-picker"
import { linkPlugin } from "./link_bar/link"
import { sectionIdPlugin, withSectionIds } from "./prosemirror/sections"

type Refs = Pick<Parameters<NewsletterEditor>[1], "surface" | "musicLookup" | "imageFrame" | "musicFrame" | "youtubeFrame" | "boxoutFrame">

interface ViewOptions {
    doc: Node
    hasMedia?: boolean
    editable?: () => boolean
    plugins?: Array<Plugin>
    uploads: Uploads
    pickFiles: PickFiles
    onChange: () => void
    onUpdate: () => void
}

export const createView = (refs: Refs, { doc, hasMedia = true, editable, plugins = [], uploads, pickFiles, onChange, onUpdate }: ViewOptions): EditorView => {
    const view: EditorView = new EditorView(refs.surface, {
        state: EditorState.create({
            doc: ensureTrailingParagraph(withSectionIds(doc)),
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
                sectionIdPlugin(),
                ...(hasMedia ? [musicPlugin(refs.musicLookup), youtubePlugin()] : []),
                boxoutPlugin(),
                linkPlugin(),
                uploads.plugin(),
                ...plugins,
            ],
        }),
        editable,
        attributes: {
            class: "flow prose-body",
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
            youtube: youtubeView(refs.youtubeFrame),
            boxout: boxoutView(refs.boxoutFrame),
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
            if (!hasMedia) {
                return false
            }
            const files = filterImageFiles(event.clipboardData?.files)
            if (files.length === 0) {
                return false
            }
            uploads.insert(editor, files)
            return true
        },
        handleDrop: (editor, event, _slice, wasMoved) => {
            if (wasMoved || !hasMedia) {
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
