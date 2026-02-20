// loosely based on / shamelessly stolen from https://github.com/HoHieuLuc/tiptap-resizable-image/blob/master/packages/tiptap-resizable-image/src/resizable-image.types.ts

export interface ImageBlockOptions {
    /** HTML attributes for the resizable image. */
    HTMLAttributes: Record<string, unknown>;
    /**
     * Allow base 64 as src.
     * @default true
     */
    allowBase64: boolean;
    /**
     * Default width of the image element.
     * @default 500
     */
    defaultWidth: number;
    /**
     * Default height of the image element.
     * @default 500
     */
    defaultHeight: number;
    /**
     * Min width that the image element can be resized to.
     * @default 100
     */
    minWidth: number;
    /**
     * Max width that the image element can be resized to.
     * @default 16384
     */
    maxWidth: number;
    /**
     * Min height that the image element can be resized to.
     * @default 100
     */
    minHeight: number;
    /**
     * Max height that the image element can be resized to.
     * @default Infinity
     */
    maxHeight: number;
    /**
     * Determines whether the caption should be displayed.
     * @default false
     */
    /** Optional function to handle uploading when pasting and dropping image into the editor. */
    onUpload?: (file: File) => Promise<ImageBlockAttributes>;
    /** Optional function to handle context menu events. */
    //   onContextMenu?: (
    //     /** The React mouse event. */
    //     event: React.MouseEvent<HTMLImageElement, MouseEvent>,
    //     /** The payload for the context menu event. */
    //     payload: ResizableImageNodeViewRendererProps
    //   ) => void;
}


interface ImageBlockAttributes {
    src: string
}


declare module '@tiptap/core' {
    interface Commands<ReturnType> {
        imageComponent: {
            insertImageBlock(
                attrs: ImageBlockAttributes | null, // It can be null if we are inserting a placeholder
                // position?: number | Range,
                // options?: InsertContentAtOptions
            ): ReturnType;
        };
    }
}