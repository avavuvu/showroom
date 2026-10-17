import { filterImageFiles } from "../../blocks/image/upload"

export type PickFiles = (callback: (files: Array<File>) => void, isMultiple: boolean) => void

export const createFilePicker = (input: HTMLInputElement): { pick: PickFiles, handlePicked: () => void } => {
    let pending: ((files: Array<File>) => void) | null = null
    return {
        pick: (callback, isMultiple) => {
            pending = callback
            input.multiple = isMultiple
            input.click()
        },
        handlePicked: () => {
            const files = filterImageFiles(input.files)
            input.value = ""
            pending?.(files)
            pending = null
        },
    }
}
