<script setup lang="ts">
import { Editor } from '@tiptap/vue-3';
import { computed, ref } from 'vue';


const { editor } = defineProps<{ editor: Editor }>()

const setLink = () => {
    const previousUrl = editor.getAttributes('link').href
    const url = window.prompt('URL', previousUrl)

    if (url === null) {
        return
    }

    if (url === '') {
        editor.chain().focus().extendMarkRange('link').unsetLink().run()

        return
    }

    editor.chain().focus().extendMarkRange('link').setLink({ href: url }).run()
}

const toggleBold = () => {
    editor.chain().focus().toggleBold().run()
}
const toggleItalic = () => {
    editor.chain().focus().toggleItalic().run()
}
const toggleStrike = () => {
    editor.chain().focus().toggleStrike().run()
}

const toggleBulletList = () => {
    editor.chain().focus().toggleBulletList().run()
}
const toggleOrderedList = () => {
    editor.chain().focus().toggleOrderedList().run()
}



const setHeading = (event: Event) => {
    const target = event.target as HTMLSelectElement
    const value = target.value

    if (value === 'paragraph') {
        editor.chain().focus().setParagraph().run()
    } else if (value) {
        editor.chain().focus().toggleHeading({ level: parseInt(value) as 1 | 2 | 3 }).run()
    }
}

const isBold = computed(() => editor?.isActive('bold') ?? false)
const isItalic = computed(() => editor?.isActive('italic') ?? false)
const isStrike = computed(() => editor?.isActive('strike') ?? false)
const isLink = computed(() => editor?.isActive('link') ?? false)

const activeHeading = computed(() => {
    if (!editor) return 'paragraph'
    return editor.isActive('heading')
        ? String(editor.getAttributes('heading').level)
        : 'paragraph'
})

const activeList = computed(() => {
    if (!editor) return false
    return editor.isActive("listItem")
})

const currentListType = ref("ol")

const toggleList = () => {
    editor.chain().focus().toggleList(
        currentListType.value === "ul" ? "bulletList" : "orderedList",
        'listItem').run()
}

const setList = (event: Event) => {
    const target = event.target as HTMLSelectElement
    const value = target.value

    currentListType.value = value

    if (value === 'ol') {
        editor.chain().focus().toggleOrderedList().run()
    } else if (value === 'ul') {
        editor.chain().focus().toggleBulletList().run()
    }
}

const addImage = () => {
    const url = window.prompt('URL')

    if (url) {
        editor.chain().focus().setImage({ src: url }).run()
    }
}

</script>

<template>
    <div v-if="editor" class="toolbar">
        <div :class="{ 'is-active': activeList }" class="border flex gap-4">
            <button @click="toggleList">
                List
            </button>
            <select @change="setList" :value="currentListType">
                <option value="ol">OL</option>
                <option value="ul">UL</option>
            </select>
        </div>

        <select @change="setHeading" :value="activeHeading">
            <option value="paragraph">Normal Text</option>
            <option value="1">Heading 1</option>
            <option value="2">Heading 2</option>
            <option value="3">Heading 3</option>
        </select>

        <button :class="{ 'is-active': isBold }" class="bold" @click="toggleBold">B</button>
        <button :class="{ 'is-active': isItalic }" class="italic" @click="toggleItalic">I</button>
        <button :class="{ 'is-active': isStrike }" class="strike" @click="toggleStrike">S</button>
        <button :class="{ 'is-active': isLink }" class="link" @click="setLink">L</button>

        <button @click="addImage">Set image</button>
    </div>
</template>

<style>
@reference "tailwindcss";

ul {
    @apply list-disc;
}

ol {
    @apply list-decimal;
}

.is-active {
    @apply bg-gray-200;
}

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