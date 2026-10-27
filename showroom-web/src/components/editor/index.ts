import { provide } from "../../../bindings/boutique/setups"
import type { EditorProps } from "../../../bindings/types/EditorProps"
import type { NewsletterEditor } from "@setups"
import type { Node } from "prosemirror-model"
import type { EditorView } from "prosemirror-view"
import { editorContext } from "./context"
import { getBodyStart } from "./prosemirror/name"
import { schema } from "./prosemirror/schema"
import { withSectionIds } from "./prosemirror/sections"
import { insertImage, Uploads } from "../../blocks/image/upload"
import { holdUnload } from "../dirty_form/unload"
import { createFilePicker } from "./file-picker"
import { SaveController } from "./save"
import { SaveStatus } from "./save_status/status"
import { createView } from "./view"
import { trackViewport } from "./viewport"

export const newsletterEditor: NewsletterEditor = (root, refs, { signal }) => {
    const props = JSON.parse(root.dataset.props ?? "{}") as EditorProps
    const status = new SaveStatus({ status: refs.status, announce: refs.saveAnnounce, alert: refs.saveAlert, actions: refs.saveActions })

    let doc: Node
    try {
        doc = schema.nodeFromJSON(props.content)
        doc.check()
    } catch (error) {
        status.showFailure("This newsletter could not be loaded. Nothing was changed.")
        refs.title.disabled = true
        refs.subtitle.disabled = true
        throw new Error("the stored document does not match the editor schema", { cause: error })
    }

    const report = (message: string) => status.showProblem(message, () => saver.state)
    const filePicker = createFilePicker(refs.fileInput)
    const uploads = new Uploads(refs.uploadPlaceholder, report)

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
    })

    const listeners = new Set<(view: EditorView) => void>()
    const view = createView(refs, {
        doc,
        uploads,
        pickFiles: filePicker.pick,
        onChange: () => saver.markDirty(),
        onUpdate: () => listeners.forEach((listener) => listener(view)),
    })
    if (!withSectionIds(doc).eq(doc)) {
        saver.markDirty()
    }

    const go = async (href: string) => {
        if (uploads.isActive) {
            report("Wait until the image upload is complete.")
            return
        }
        const isSaved = await saver.flush()
        if (isSaved || window.confirm("Your latest changes are not saved. Leave this page?")) {
            saver.discard()
            window.location.assign(href)
        }
    }

    const base = `/${encodeURIComponent(props.slug)}/edit/${encodeURIComponent(props.id)}`

    provide(root, editorContext, {
        view,
        reviews: {
            summaries: props.reviews,
            sendUrl: (sectionId) => `${base}/review?section=${encodeURIComponent(sectionId)}`,
            listUrl: `${base}/reviews`,
            navigate: (href) => void go(href),
        },
        insertImage: insertImage(uploads, filePicker.pick),
        onUpdate: (listener, signal) => {
            listeners.add(listener)
            signal.addEventListener("abort", () => listeners.delete(listener), { once: true })
            listener(view)
        },
    })

    const stopViewport = trackViewport(root, root.querySelector<HTMLElement>(":scope > .workspace-header"), refs.toolbar, view, signal)

    return {
        filesPicked: () => filePicker.handlePicked(),

        markDirty: () => saver.markDirty(),

        toSubtitle: () => refs.subtitle.focus(),

        toBody: () => {
            view.dispatch(view.state.tr.setSelection(getBodyStart(view.state.doc)))
            view.focus()
        },

        save: () => void saver.flush(),

        guard: (event) => holdUnload(event, saver.hasUnsavedChanges || uploads.isActive),

        leave: async (event, link) => {
            if (!(link instanceof HTMLAnchorElement) || !link.href || event.defaultPrevented) {
                return
            }
            event.preventDefault()
            await go(link.href)
        },

        restore: (event) => {
            if (event.persisted && saver.isDiscarded) {
                window.location.reload()
            }
        },

        reload: () => {
            saver.discard()
            window.location.reload()
        },

        overwrite: () => void saver.overwrite(),

        disconnect: () => {
            status.stop()
            stopViewport()
            saver.stop()
            view.destroy()
        },
    }
}
export { reviewEditor } from "./review"
