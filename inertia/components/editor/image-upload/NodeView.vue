<script setup lang="ts">
import { NodeViewWrapper } from '@tiptap/vue-3'
import { nodeViewProps } from '@tiptap/vue-3'

const { node, updateAttributes, selected, deleteNode } = defineProps(nodeViewProps)

const handleUpload = () => {
    updateAttributes({
        src: "https://substackcdn.com/image/fetch/$s_!legx!,w_1456,c_limit,f_webp,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F92f8d7fe-aa02-421e-b378-d0c4eab53f40_1920x1080.webp"
    })
}

const handleWidth = () => {
    updateAttributes({
        width: node.attrs.width === "full" ? "normal" : "full"
    })
}

const handleDelete = () => {
    deleteNode()
}


</script>

<template>
    <node-view-wrapper class="image-upload-block" :data-width="node.attrs.width">
        <div v-if="!node.attrs.src" class="upload-container" data-drag-handle>
            <button class="upload-button" @click="handleUpload">

                Upload Image
            </button>
        </div>
        <div v-else class="image-container" data-drag-handle>
            <img :src="node.attrs.src" />
            <div v-if="selected" class="image-toolbar">
                <button @click.stop="">
                    Replace
                </button>
                <button @click.stop="handleDelete">
                    Delete
                </button>
                <button @click.stop="handleWidth">
                    {{ node.attrs.width === "full" ? "Normal Width" : "Full Width" }}
                </button>
            </div>
        </div>
    </node-view-wrapper>
</template>

<style scoped>
.image-upload-block {
    position: relative;
}

.upload-container {
    aspect-ratio: 16 / 9;
    border: 1px dashed var(--color-border);
    display: flex;
    width: 100%;
    justify-content: center;
    align-items: center;
    transition: all 300ms;
    background-color: var(--color-surface);
    cursor: move;

    &:hover {
        background-color: var(--color-surface-subtle);
        border-color: var(--color-border-hover);
    }
}

.upload-button {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 24px 32px;
    border: 1px solid black;
    font-size: 16px;
    font-weight: 500;
    cursor: pointer;
    transition: all 200ms;

    &:hover {
        background: var(--color-surface-subtle);
    }

    &:active {
        transform: translateY(0);
    }

    svg {
        width: 32px;
        height: 32px;
    }
}

.image-container {
    position: relative;
    cursor: move;

    img {
        width: 100%;
        height: auto;
    }
}

.image-toolbar {
    position: absolute;
    top: 8px;
    right: 8px;
    display: flex;
    gap: 4px;
    background: var(--color-white);
    border: 1px solid var(--color-border);
    z-index: 10;

    button {
        display: flex;
        align-items: center;
        gap: 4px;
        padding: 6px 10px;
        border: none;
        background: transparent;
        cursor: pointer;
        font-size: 14px;
        transition: background 150ms;
    }

    button:hover {
        background-color: var(--color-surface-subtle);
    }
}
</style>