import { inject } from "../../../../bindings/boutique/setups"
import type { EditorLinkBar } from "@setups"
import { editorContext } from "../context"
import { getSelectedName, isEditingName, setNameFallback, stopEditingName } from "../prosemirror/name"
import { applyLink, closeLink, getLinkEditing, getLinkText, getSelectedLink, hasLinkChanges, openLink, removeLink } from "./link"

export const editorLinkBar: EditorLinkBar = async (root, { text, url, fallback, open, remove }, { signal }) => {
    const { view, onUpdate } = await inject(root, editorContext).catch((error: unknown) => {
        text.disabled = true
        url.disabled = true
        fallback.disabled = true
        throw error
    })

    let wasEditing = false
    let wasEditingName = false
    let originalFallback = ""

    const sync = () => {
        const { state } = view
        const editing = getLinkEditing(state)
        const range = getSelectedLink(state)
        const target = editing ?? range
        const href = editing?.href || (range?.mark.attrs.href as string | undefined) || ""
        const appliedHref = editing ? editing.appliedHref : ((range?.mark.attrs.href as string | undefined) ?? "")
        const isFocused = root.contains(document.activeElement)
        const selectedName = getSelectedName(state)
        const isNameEditing = isEditingName(state)

        root.classList.toggle("is-idle", !target && !selectedName)
        root.classList.toggle("is-name", !!selectedName)
        if (selectedName && document.activeElement !== fallback) {
            fallback.value = selectedName.fallback
        }
        if (isNameEditing && !wasEditingName && document.activeElement !== fallback) {
            requestAnimationFrame(() => {
                fallback.focus()
                fallback.select()
            })
        }
        wasEditingName = isNameEditing
        if (appliedHref) {
            open.href = appliedHref
        } else {
            open.removeAttribute("href")
        }
        remove.disabled = !appliedHref

        if (!isFocused) {
            text.value = target ? getLinkText(state, target) : ""
            url.value = href
            url.setCustomValidity("")
        }

        if (editing && !wasEditing && !isFocused) {
            const firstField = editing.from === editing.to ? text : url
            requestAnimationFrame(() => {
                firstField.focus()
                firstField.select()
            })
        }
        wasEditing = !!editing
    }

    onUpdate(sync, signal)

    return {
        start: () => {
            if (!getLinkEditing(view.state)) {
                openLink(view.state, view.dispatch, view)
            }
        },

        leave: () => {
            const href = url.value
            const label = text.value
            requestAnimationFrame(() => {
                if (view.isDestroyed || root.contains(document.activeElement)) {
                    return
                }
                if (!hasLinkChanges(view.state, href, label) || applyLink(view, href, label, { shouldMoveCursor: false })) {
                    closeLink(view)
                }
                stopEditingName(view)
                sync()
            })
        },

        cancel: () => {
            closeLink(view)
            view.focus()
        },

        apply: () => {
            const problem = applyLink(view, url.value, text.value)
            if (!problem) {
                view.focus()
                return
            }
            url.setCustomValidity(problem)
            url.reportValidity()
        },

        clearProblem: () => url.setCustomValidity(""),

        startName: () => {
            originalFallback = getSelectedName(view.state)?.fallback ?? ""
        },

        changeFallback: () => setNameFallback(view, fallback.value.trim()),

        finishName: () => {
            stopEditingName(view)
            view.focus()
        },

        cancelName: () => {
            setNameFallback(view, originalFallback)
            stopEditingName(view)
            view.focus()
        },

        unlink: () => {
            removeLink(view)
            view.focus()
        },
    }
}
