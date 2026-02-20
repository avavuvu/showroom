import { mergeAttributes, Node } from '@tiptap/core'
import { VueNodeViewRenderer } from '@tiptap/vue-3'
import NodeView from "./NodeView.vue"
import { ImageBlockOptions } from './image-upload.types'
import { Plugin, PluginKey } from '@tiptap/pm/state'

export default Node.create<ImageBlockOptions>({
    name: 'imageBlock',
    group: 'block',
    inline: false,
    draggable: true,
    atom: true,

    addAttributes() {
        return {
            src: null,
            width: {
                default: 'normal',
                parseHTML: element => element.getAttribute('data-width') || 'normal',
                renderHTML: attributes => {
                    return {
                        'data-width': attributes.width,
                    }
                },
            },
        }
    },

    parseHTML() {
        return [
            {
                tag: 'div[data-type="image-block"]',
            },
        ]
    },

    addCommands() {
        return {
            insertImageBlock: (attributes) => ({ commands }) => {
                return commands.insertContentAt(
                    this.editor.state.selection.head,
                    {
                        type: 'imageBlock',
                        attrs: {
                            src: attributes?.src || null,
                            width: "normal"
                        },
                    },
                )
            },
        }
    },

    renderHTML({ HTMLAttributes }) {
        return [
            'div',
            mergeAttributes(HTMLAttributes, { 'data-type': 'image-block' }),
        ]
    },

    addNodeView() {
        return VueNodeViewRenderer(NodeView)
    },

    addProseMirrorPlugins() {
        return [
            new Plugin({
                key: new PluginKey(this.name),
                props: {
                    handleDrop: (view, dragEvent) => {
                        dragEvent.preventDefault();
                        const files = dragEvent.dataTransfer?.files;

                        const isDroppedFromFileSystem =
                            !dragEvent.dataTransfer?.getData('text/html');

                        if (
                            !files ||
                            files.length === 0 ||
                            !this.options.onUpload ||
                            !isDroppedFromFileSystem
                        ) {
                            return false;
                        }

                        const coordinates = view.posAtCoords({
                            left: dragEvent.clientX,
                            top: dragEvent.clientY,
                        });

                        for (const file of files) {
                            this.options
                                .onUpload(file)
                                .then((attributes) => { //this will equal something one day
                                    this.editor
                                        .chain()
                                        .focus()
                                        .insertImageBlock(attributes)
                                        .run();
                                });
                        }
                    }
                }
            })
        ]
    }
})