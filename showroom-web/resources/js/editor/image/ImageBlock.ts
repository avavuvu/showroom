import Image from "@tiptap/extension-image";
import { VueNodeViewRenderer } from "@tiptap/vue-3";
import { Plugin, PluginKey } from "@tiptap/pm/state";
import ImageBlockView from "./ImageBlockView.vue";
import { uploadImage } from "./uploadImage";

export const ImageBlock = Image.extend({
    draggable: true,

    addAttributes() {
        return {
            ...this.parent?.(),
            width: {
                default: "normal",
                parseHTML: (element) => element.getAttribute("data-width") || "normal",
                renderHTML: (attributes) => ({ "data-width": attributes.width }),
            },
            publicId: {
                default: null,
                parseHTML: (element) => element.getAttribute("data-public-id"),
                renderHTML: (attributes) =>
                    attributes.publicId ? { "data-public-id": attributes.publicId } : {},
            },
        };
    },

    addNodeView() {
        return VueNodeViewRenderer(ImageBlockView);
    },

    addProseMirrorPlugins() {
        return [
            new Plugin({
                key: new PluginKey("imageBlockDrop"),
                props: {
                    handleDrop: (view, dragEvent) => {
                        const files = dragEvent.dataTransfer?.files;
                        const isDroppedFromFileSystem = !dragEvent.dataTransfer?.getData("text/html");

                        if (!files || files.length === 0 || !isDroppedFromFileSystem) {
                            return false;
                        }

                        dragEvent.preventDefault();

                        const coordinates = view.posAtCoords({
                            left: dragEvent.clientX,
                            top: dragEvent.clientY,
                        });

                        for (const file of files) {
                            if (!file.type.startsWith("image/")) continue;

                            uploadImage(file).then(({ src, publicId }) => {
                                const { schema } = view.state;
                                const node = schema.nodes.image.create({ src, alt: file.name, publicId });
                                const pos = coordinates?.pos ?? view.state.selection.head;
                                view.dispatch(view.state.tr.insert(pos, node));
                            });
                        }

                        return true;
                    },
                },
            }),
        ];
    },
});

export default ImageBlock;
