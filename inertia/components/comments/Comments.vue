<script setup lang="ts">
import type Comment from '#models/comment'
import { computed } from 'vue';
import CommentList from './CommentList.vue'
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import { Placeholder } from '@tiptap/extensions';
import { useForm, usePage } from '@inertiajs/vue3';
import Toolbar from '../editor/Toolbar.vue';
import Button from '../blocks/Button.vue';
import { SharedProps } from '@adonisjs/inertia/types';
import Dialog from '../blocks/Dialog.vue';
import QuickRegister from '../auth/QuickRegister.vue';

const { user } = usePage<SharedProps>().props

const props = defineProps<{ comments?: Comment[] }>()

const rootComments = computed(() => props.comments?.filter(c => c.parentId === null) || [])

const form = useForm({
    content: "",
    userId: user?.id || null,
})

const onsubmit = (event: Event) => {
    event.preventDefault()

    form.post("comment")
}

const editor = useEditor({
    content: form.content || "",
    extensions: [
        StarterKit.configure({
            blockquote: false,
            link: false,
            heading: false,
            bulletList: false,
            orderedList: false,
            code: false,
            dropcursor: false,
        }),
        Placeholder.configure({
            placeholder: "Leave a comment..."
        }),
    ],
    editorProps: {
        attributes: {
            class: "outline-none !block min-h-32 px-2 ",
        },
    },
})
</script>

<template>
    <div class="border p-4 min-h-48">
        <div class="inline-flex justify-between w-full">
            <Toolbar v-if="editor" :editor variant="minimal" />

            <Button v-if="user" @click="onsubmit" size="sm">Post</Button>
            <Dialog v-else lazy as-child title="Sign up to Showroom">
                <template #trigger>
                    <Button size="sm">Post</Button>
                </template>
                <QuickRegister />
            </Dialog>
        </div>
        <editor-content :editor>
        </editor-content>
    </div>



    <CommentList :comments="rootComments" :all-comments="comments" />
</template>