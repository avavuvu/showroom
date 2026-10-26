import { context } from "../../../../bindings/boutique/setups"
import type { BoxoutAttrs } from "../../../../bindings/types/BoxoutAttrs"

export interface BoxoutHandle {
    getAttrs(): BoxoutAttrs
    change(fields: Partial<BoxoutAttrs>): void
    unwrap(): void
    remove(): void
    getColumn(): DOMRect
    onUpdate(listener: () => void, signal: AbortSignal): void
}

export const boxoutEditorKey = context<BoxoutHandle>("boxout-editor")
