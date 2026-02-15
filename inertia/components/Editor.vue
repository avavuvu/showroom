<script setup lang="ts">
import { EditorContent, Editor } from '@tiptap/vue-3'
import { watch } from 'vue'

const { modelValue, editor } = defineProps<{
    modelValue?: string,
    editor: Editor
}>()

const emit = defineEmits<{
    (e: 'update:modelValue', value: string): void
}>()

watch(() => modelValue, (value) => {
    if (!editor) return

    const isSame = editor.getHTML() === value

    if (isSame) {
        return
    }

    editor.commands.setContent(value || '')
})
</script>

<template>
    <editor-content :editor="editor" />
</template>