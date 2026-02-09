<script setup lang="ts">
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'

const props = defineProps<{
    modelValue: string
}>()

const emit = defineEmits<{
    (e: 'update:modelValue', value: string): void
}>()

const editor = useEditor({
    content: props.modelValue,
    extensions: [StarterKit],
    editorProps: {
        attributes: {
            class: 'prose prose-sm sm:prose mx-auto focus:outline-none',
        },
    },
})

const setLink = () => {
    const previousUrl = editor.value?.getAttributes('link').href
    const url = window.prompt('URL', previousUrl)

    if (url === null) {
        return
    }

    if (url === '') {
        editor.value?.chain().focus().extendMarkRange('link').unsetLink().run()

        return
    }

    editor.value?.chain().focus().extendMarkRange('link').setLink({ href: url }).run()
}

const toggleBold = () => {
    editor.value?.chain().focus().toggleBold().run()
}
const toggleItalic = () => {
    editor.value?.chain().focus().toggleItalic().run()
}
const toggleStrike = () => {
    editor.value?.chain().focus().toggleStrike().run()
}

const toggleBulletList = () => {
    editor.value?.chain().focus().toggleBulletList().run()
}
const toggleOrderedList = () => {
    editor.value?.chain().focus().toggleOrderedList().run()
}

const setHeading = (event: Event) => {
    const target = event.target as HTMLSelectElement
    const value = target.value

    if (value === 'paragraph') {
        editor.value?.chain().focus().setParagraph().run()
    } else if (value) {
        editor.value?.chain().focus().toggleHeading({ level: parseInt(value) as 1 | 2 | 3 }).run()
    }
}
</script>

<template>
    <div v-if="editor" class="toolbar">
        <select @change="setHeading">
            <option value="paragraph">Normal Text</option>
            <option value="1">Heading 1</option>
            <option value="2">Heading 2</option>
            <option value="3">Heading 3</option>
        </select>

        <button class=" link" @click="setLink">L</button>
        <button class="bold" @click="toggleBold">B</button>
        <button class=" italic" @click="toggleItalic">I</button>

        <button class="strike" @click="toggleStrike">S</button>

        <button @click="toggleBulletList">Unordered List</button>
        <button @click="toggleOrderedList">Ordered List</button>
    </div>

    <editor-content :editor="editor" />
</template>

<style>
@reference "tailwindcss";

.toolbar {
    @apply grid grid-cols-5 gap-2;
}


u,
.toolbar .underline {
    text-decoration: underline;
}

em,
.toolbar .italic {
    font-style: italic;
}

strong,
.toolbar .bold {
    font-weight: bold;
}

.tiptap a,
.toolbar .link {
    text-decoration: underline;
    color: blue;

}
</style>