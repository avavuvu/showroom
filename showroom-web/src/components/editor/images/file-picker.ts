import { imageFiles } from "./upload";

export type PickFiles = (callback: (files: File[]) => void, multiple: boolean) => void;

export class FilePicker {
    private pending: ((files: File[]) => void) | null = null;

    constructor(private input: HTMLInputElement) {}

    pick: PickFiles = (callback, multiple) => {
        this.pending = callback;
        this.input.multiple = multiple;
        this.input.click();
    };

    picked() {
        const files = imageFiles(this.input.files);
        this.input.value = "";
        this.pending?.(files);
        this.pending = null;
    }
}
