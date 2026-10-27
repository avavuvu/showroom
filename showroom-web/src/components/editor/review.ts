import { provide } from "../../../bindings/boutique/setups"
import type { ReviewProps } from "../../../bindings/types/ReviewProps"
import type { ReviewSaveRequest } from "../../../bindings/types/ReviewSaveRequest"
import type { ReviewSaveResponse } from "../../../bindings/types/ReviewSaveResponse"
import type { ReviewEditor } from "@setups"
import type { Node } from "prosemirror-model"
import { Plugin } from "prosemirror-state"
import type { EditorView } from "prosemirror-view"
import { Uploads } from "../../blocks/image/upload"
import { holdUnload } from "../dirty_form/unload"
import { editorContext } from "./context"
import { schema } from "./prosemirror/schema"
import { SaveController } from "./save"
import { SaveStatus } from "./save_status/status"
import { markersPlugin } from "./reviews/markers"
import { createView } from "./view"
import { trackViewport } from "./viewport"

const keepHeading = (sectionId: string) => new Plugin({
    filterTransaction: (transaction) => {
        if (!transaction.docChanged) {
            return true
        }
        const first = transaction.doc.firstChild
        return first?.type === schema.nodes.heading && first.attrs.id === sectionId
    },
})

const readError = async (response: Response): Promise<string> => {
    if (response.status === 403 || response.status === 404) {
        return "This link is no longer active."
    }
    if (response.status === 409) {
        return "This changed in another tab. Reload, then send again."
    }
    const text = await response.text().catch(() => "")
    const message = new DOMParser().parseFromString(text, "text/html").body.textContent?.trim()
    return message || "Your suggestions could not be sent. Try again."
}

export const reviewEditor: ReviewEditor = (root, refs, { signal }) => {
    const props = JSON.parse(root.dataset.props ?? "{}") as ReviewProps
    const status = new SaveStatus(
        { status: refs.status, announce: refs.saveAnnounce, alert: refs.saveAlert, actions: refs.saveActions },
        {
            conflict: "This section changed in another tab or window.",
            expired: "This link is no longer active, so changes can’t be saved.",
        },
    )
    let doc: Node
    let base: Node
    try {
        doc = schema.nodeFromJSON(props.content)
        doc.check()
        base = schema.nodeFromJSON(props.base)
    } catch (error) {
        status.showFailure("This section could not be loaded.")
        throw new Error("the review content does not match the editor schema", { cause: error })
    }

    const report = (message: string) => status.showProblem(message, () => saver.state)
    const uploads = new Uploads(refs.uploadPlaceholder, report)

    const saver = new SaveController<Omit<ReviewSaveRequest, "revision">>({
        url: props.url,
        revision: props.revision,
        read: () => ({ content: view.state.doc.toJSON() }),
        onChange: (state) => status.render(state),
        signal,
    })

    let isSent = false
    const markUnsent = () => {
        if (isSent && refs.send) {
            isSent = false
            refs.send.textContent = "Send update"
            refs.send.toggleAttribute("disabled", false)
        }
    }

    const listeners = new Set<(view: EditorView) => void>()
    const view = createView(refs, {
        doc,
        hasMedia: false,
        plugins: [markersPlugin(base), ...(props.sectionId ? [keepHeading(props.sectionId)] : [])],
        uploads,
        pickFiles: () => undefined,
        onChange: () => {
            saver.markDirty()
            markUnsent()
        },
        onUpdate: () => listeners.forEach((listener) => listener(view)),
    })

    provide(root, editorContext, {
        view,
        insertImage: () => false,
        onUpdate: (listener, listenerSignal) => {
            listeners.add(listener)
            listenerSignal.addEventListener("abort", () => listeners.delete(listener), { once: true })
            listener(view)
        },
    })

    const stopViewport = trackViewport(root, root.querySelector<HTMLElement>(":scope > .workspace-header"), refs.toolbar, view, signal)

    return {
        save: () => void saver.flush(),

        guard: (event) => holdUnload(event, saver.hasUnsavedChanges),

        reload: () => {
            saver.discard()
            window.location.reload()
        },

        overwrite: () => void saver.overwrite(),

        replyEdited: markUnsent,

        send: async () => {
            const send = refs.send
            if (isSent) {
                return
            }
            send?.toggleAttribute("disabled", true)
            refs.sendError.textContent = ""
            try {
                await saver.flush()
                const body: ReviewSaveRequest = {
                    content: view.state.doc.toJSON(),
                    revision: saver.currentRevision,
                    reply: refs.reply.value.trim() || undefined,
                }
                const response = await fetch(`${props.url}/send`, {
                    method: "POST",
                    signal,
                    headers: { "Content-Type": "application/json", Accept: "application/json" },
                    body: JSON.stringify(body),
                })
                if (!response.ok) {
                    refs.sendError.textContent = await readError(response)
                    return
                }
                saver.acceptRevision(((await response.json()) as ReviewSaveResponse).revision)
                refs.sentStatus.textContent = ""
                if (send) {
                    send.textContent = "Sent"
                    isSent = true
                }
            } catch {
                refs.sendError.textContent = "Your suggestions could not be sent. Check your connection and try again."
            } finally {
                send?.toggleAttribute("disabled", isSent)
            }
        },

        disconnect: () => {
            status.stop()
            stopViewport()
            saver.stop()
            view.destroy()
        },
    }
}
