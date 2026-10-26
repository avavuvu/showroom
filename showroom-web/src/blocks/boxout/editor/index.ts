import type { BoxoutEditor } from "@setups"
import { inject } from "../../../../bindings/boutique/setups"
import type { BoxoutWidth } from "../../../../bindings/types/BoxoutWidth"
import { readBoxoutStyle } from "../schema"
import { boxoutEditorKey } from "./handle"

const WIDE_FRACTION = 1.02
const TEXT_FRACTION = 0.7

const pickWidth = (fraction: number, side: BoxoutWidth): BoxoutWidth => {
    if (fraction >= WIDE_FRACTION) {
        return "wide"
    }
    if (fraction >= TEXT_FRACTION) {
        return "text"
    }
    return side
}

const setPressed = (buttons: Array<HTMLButtonElement>, isPressed: (button: HTMLButtonElement) => boolean) => {
    for (const button of buttons) {
        button.setAttribute("aria-pressed", String(isPressed(button)))
    }
}

export const boxoutEditor: BoxoutEditor = async (root, refs, { signal }) => {
    const boxout = await inject(root, boxoutEditorKey)

    boxout.onUpdate(() => {
        const { style } = boxout.getAttrs()
        setPressed(refs.style, (button) => button.dataset.style === style)
    }, signal)

    return {
        setStyle: (_event, button) => boxout.change({ style: readBoxoutStyle(button.dataset.style) }),
        unwrap: () => boxout.unwrap(),
        remove: () => boxout.remove(),
        drag: (event, handle) => {
            if (event.button !== 0) {
                return
            }
            event.preventDefault()
            const isStart = handle.dataset.edge === "start"
            const column = boxout.getColumn()
            let width = boxout.getAttrs().width

            const measure = (clientX: number): BoxoutWidth => {
                const fraction = isStart ? (column.right - clientX) / column.width : (clientX - column.left) / column.width
                return pickWidth(fraction, isStart ? "right" : "left")
            }
            const move = (moveEvent: PointerEvent) => {
                width = measure(moveEvent.clientX)
                root.dataset.width = width
            }
            const dragController = new AbortController()
            const abortDrag = () => dragController.abort()
            signal.addEventListener("abort", abortDrag, { once: true })
            const finish = () => {
                if (dragController.signal.aborted) {
                    return
                }
                dragController.abort()
                signal.removeEventListener("abort", abortDrag)
                root.classList.remove("is-resizing")
                boxout.change({ width })
            }

            handle.setPointerCapture(event.pointerId)
            root.classList.add("is-resizing")
            const dragListeners = { signal: dragController.signal }
            handle.addEventListener("pointermove", move, dragListeners)
            handle.addEventListener("pointerup", finish, dragListeners)
            handle.addEventListener("pointercancel", finish, dragListeners)
            handle.addEventListener("lostpointercapture", finish, dragListeners)
        },
    }
}
