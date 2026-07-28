<script setup lang="ts">
import { NodeViewWrapper, nodeViewProps } from "@tiptap/vue-3";
import { ref } from "vue";
import { readImageAsDataUrl } from "./readImageAsDataUrl";

const { node, updateAttributes, selected, deleteNode } = defineProps(nodeViewProps);

const fileInput = ref<HTMLInputElement | null>(null);
const isLoading = ref(false);
const loadError = ref<string | null>(null);

async function loadFile(file: File) {
    if (!file.type.startsWith("image/")) {
        loadError.value = "File must be an image";
        return;
    }

    isLoading.value = true;
    loadError.value = null;

    try {
        const src = await readImageAsDataUrl(file);
        updateAttributes({ src, alt: file.name });
    } catch {
        loadError.value = "Could not read file";
    } finally {
        isLoading.value = false;
    }
}

function openPicker() {
    fileInput.value?.click();
}

function onFileSelected(event: Event) {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;
    loadFile(file);
    (event.target as HTMLInputElement).value = "";
}

function toggleWidth() {
    updateAttributes({ width: node.attrs.width === "full" ? "normal" : "full" });
}

function handleDelete() {
    deleteNode();
}
</script>

<template>
    <node-view-wrapper class="image-upload-block" :data-width="node.attrs.width">
        <input
            ref="fileInput"
            type="file"
            accept="image/*"
            class="hidden-file-input"
            @change="onFileSelected"
        />

        <div v-if="!node.attrs.src && !isLoading && !loadError" class="upload-container" data-drag-handle>
            <button class="upload-button" @click="openPicker">Upload Image</button>
        </div>

        <div v-else-if="isLoading" class="upload-container" data-drag-handle>
            <span>Loading...</span>
        </div>

        <div v-else-if="loadError" class="upload-container upload-error" data-drag-handle>
            <span class="error-message">{{ loadError }}</span>
            <button class="upload-button" @click="openPicker">Try again</button>
        </div>

        <div v-else class="image-container" data-drag-handle>
            <img :src="node.attrs.src" :alt="node.attrs.alt" />
            <div v-if="selected" class="image-toolbar">
                <button @click.stop="openPicker">Replace</button>
                <button @click.stop="handleDelete">Delete</button>
                <button @click.stop="toggleWidth">
                    {{ node.attrs.width === "full" ? "Normal Width" : "Full Width" }}
                </button>
            </div>
        </div>
    </node-view-wrapper>
</template>

<style scoped>
.hidden-file-input {
    display: none;
}

.image-upload-block {
    position: relative;
}

.upload-container {
    aspect-ratio: 16 / 9;
    border: 1px dashed var(--color-border);
    display: flex;
    flex-direction: column;
    width: 100%;
    justify-content: center;
    align-items: center;
    gap: 0.75em;
    background-color: var(--color-surface);
    cursor: move;

    &:hover {
        background-color: var(--color-surface-muted);
    }
}

.upload-error {
    border-color: var(--color-error);
}

.error-message {
    font-size: 0.875em;
    color: var(--color-error);
}

.upload-button {
    padding: 0.75em 1.5em;
    border: 1px solid var(--color-foreground);
    background: var(--color-surface);
    font-size: 1em;
    cursor: pointer;

    &:hover {
        background: var(--color-surface-muted);
    }
}

.image-container {
    position: relative;
    cursor: move;

    & img {
        width: 100%;
        height: auto;
        display: block;
    }
}

.image-toolbar {
    position: absolute;
    top: 0.5em;
    right: 0.5em;
    display: flex;
    gap: 0.25em;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    z-index: 10;

    & button {
        padding: 0.4em 0.6em;
        border: none;
        background: transparent;
        cursor: pointer;
        font-size: 0.8em;

        &:hover {
            background-color: var(--color-surface-muted);
        }
    }
}
</style>
