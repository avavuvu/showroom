<script setup lang="ts">
import { Editor } from '@tiptap/vue-3';
import { computed } from 'vue';
import ToolbarItem from './ToolbarItem.vue';
import ToolbarDropdown, { type ToolbarDropdownItem } from './ToolbarDropdown.vue';
import {
    Bold, Italic, Strikethrough, Underline, Link as LinkIcon, Image as ImageIcon,
    List, ListOrdered, Undo, Redo, Heading1, Heading2, Heading3, Pilcrow, Quote, TextQuote
} from 'lucide-vue-next';

const props = defineProps<{ editor: Editor }>();

const isBold = computed(() => props.editor?.isActive('bold') ?? false);
const isItalic = computed(() => props.editor?.isActive('italic') ?? false);
const isStrike = computed(() => props.editor?.isActive('strike') ?? false);
const isUnderline = computed(() => props.editor?.isActive('underline') ?? false);
const isLink = computed(() => props.editor?.isActive('link') ?? false);
const isBulletList = computed(() => props.editor?.isActive('bulletList') ?? false);
const isOrderedList = computed(() => props.editor?.isActive('orderedList') ?? false);

const activeHeading = computed(() => {
    if (props.editor?.isActive('heading', { level: 1 })) return 'Heading 1';
    if (props.editor?.isActive('heading', { level: 2 })) return 'Heading 2';
    if (props.editor?.isActive('heading', { level: 3 })) return 'Heading 3';
    return 'Normal';
});

const activeQuote = computed(() => {
    if (props.editor?.isActive('blockquote', { class: 'pullquote' })) return 'Pull Quote';
    if (props.editor?.isActive('blockquote', { class: 'stdquote' })) return 'Block Quote';
    return null;
});

const headingItems = computed<ToolbarDropdownItem[]>(() => [
    {
        id: 'paragraph',
        label: 'Normal Text',
        icon: Pilcrow,
        action: () => props.editor.chain().focus().setParagraph().run(),
        isActive: props.editor.isActive('paragraph')
    },
    {
        id: 'h1',
        label: 'Heading 1',
        icon: Heading1,
        action: () => props.editor.chain().focus().toggleHeading({ level: 1 }).run(),
        isActive: props.editor.isActive('heading', { level: 1 })
    },
    {
        id: 'h2',
        label: 'Heading 2',
        icon: Heading2,
        action: () => props.editor.chain().focus().toggleHeading({ level: 2 }).run(),
        isActive: props.editor.isActive('heading', { level: 2 })
    },
    {
        id: 'h3',
        label: 'Heading 3',
        icon: Heading3,
        action: () => props.editor.chain().focus().toggleHeading({ level: 3 }).run(),
        isActive: props.editor.isActive('heading', { level: 3 })
    },
]);

const quoteItems = computed<ToolbarDropdownItem[]>(() => [
    {
        id: 'blockquote',
        label: 'Block Quote',
        icon: Quote,
        action: () => {
            const isActive = props.editor.isActive('blockquote', { class: 'stdquote' });
            if (isActive) {
                return props.editor.chain().focus().unsetBlockquote().run();
            }
            if (props.editor.isActive('blockquote')) {
                return props.editor.chain().focus().updateAttributes('blockquote', { class: 'stdquote' }).run();
            }
            return props.editor.chain().focus().setBlockquote().updateAttributes('blockquote', { class: 'stdquote' }).run();
        },
        isActive: props.editor.isActive('blockquote', { class: 'stdquote' })
    },
    {
        id: 'pullquote',
        label: 'Pull Quote',
        icon: TextQuote,
        action: () => {
            const isActive = props.editor.isActive('blockquote', { class: 'pullquote' });
            if (isActive) {
                return props.editor.chain().focus().unsetBlockquote().run();
            }
            if (props.editor.isActive('blockquote')) {
                return props.editor.chain().focus().updateAttributes('blockquote', { class: 'pullquote' }).run();
            }
            return props.editor.chain().focus().setBlockquote().updateAttributes('blockquote', { class: 'pullquote' }).run();
        },
        isActive: props.editor.isActive('blockquote', { class: 'pullquote' })
    },
]);

const setLink = () => {
    const previousUrl = props.editor.getAttributes('link').href;
    const url = window.prompt('URL', previousUrl);

    if (url === null) return;

    if (url === '') {
        props.editor.chain().focus().extendMarkRange('link').unsetLink().run();
        return;
    }

    props.editor.chain().focus().extendMarkRange('link').setLink({ href: url }).run();
};

const addImage = () => {
    const url = window.prompt('URL');
    if (url) {
        props.editor.chain().focus().setImage({ src: url }).run();
    }
};

</script>

<template>
    <div v-if="editor" class="flex items-center gap-1 px-2 bg-white sticky top-0 z-10">
        <!-- Undo / Redo -->
        <div class="flex items-center gap-0.5 pr-2 border-r border-border">
            <ToolbarItem :icon="Undo" @click="editor.chain().focus().undo().run()" :disabled="!editor.can().undo()"
                title="Undo" />
            <ToolbarItem :icon="Redo" @click="editor.chain().focus().redo().run()" :disabled="!editor.can().redo()"
                title="Redo" />
        </div>

        <!-- Headings -->
        <div class="px-2 border-r border-border flex gap-1">
            <ToolbarDropdown class="w-24" :items="headingItems" :label="activeHeading" />
            <ToolbarDropdown :items="quoteItems" :icon="Quote" :is-active="!!activeQuote" />
        </div>

        <!-- Formatting -->
        <div class="flex items-center gap-0.5 px-2 border-r border-border">
            <ToolbarItem :icon="Bold" @click="editor.chain().focus().toggleBold().run()" :is-active="isBold"
                title="Bold" />
            <ToolbarItem :icon="Strikethrough" @click="editor.chain().focus().toggleStrike().run()"
                :is-active="isStrike" title="Strike" />
            <ToolbarItem :icon="Underline" @click="editor.chain().focus().toggleUnderline().run()"
                :is-active="isUnderline" title="Underline" />
            <ToolbarItem :icon="Italic" @click="editor.chain().focus().toggleItalic().run()" :is-active="isItalic"
                title="Italic" />
        </div>

        <!-- Inserts -->
        <div class="flex items-center gap-0.5 px-2 border-r border-border">
            <ToolbarItem :icon="LinkIcon" @click="setLink" :is-active="isLink" title="Link" />
            <ToolbarItem :icon="ImageIcon" @click="addImage" title="Image" />
        </div>

        <!-- Lists -->
        <div class="flex items-center gap-0.5 pl-2">
            <ToolbarItem :icon="List" @click="editor.chain().focus().toggleBulletList().run()" :is-active="isBulletList"
                title="Bullet List" />
            <ToolbarItem :icon="ListOrdered" @click="editor.chain().focus().toggleOrderedList().run()"
                :is-active="isOrderedList" title="Ordered List" />
        </div>
    </div>
</template>