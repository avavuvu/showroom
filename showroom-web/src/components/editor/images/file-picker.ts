import { imageFiles } from "./upload";

export type PickFiles = (callback: (files: File[]) => void, multiple: boolean) => void;

export function filePicker(input: HTMLInputElement): { pick: PickFiles; picked: () => void } {
    let pending: ((files: File[]) => void) | null = null;
    return {
        pick(callback, multiple) {
            pending = callback;
            input.multiple = multiple;
            input.click();
        },
        picked() {
            const files = imageFiles(input.files);
            input.value = "";
            pending?.(files);
            pending = null;
        },
    };
}
