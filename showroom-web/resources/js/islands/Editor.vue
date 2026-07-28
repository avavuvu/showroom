<script setup lang="ts">
import { useEditor, EditorContent } from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";
import UnderlineExtension from "@tiptap/extension-underline";
import Link from "@tiptap/extension-link";
import ImageBlock from "../editor/image/ImageBlock";
import { onMounted, ref, watch } from "vue";
import Toolbar from "../editor/Toolbar.vue";
import { useSave } from "../editor/useSave";
import { onKeyStroke, useEventListener } from "@vueuse/core";

const props = defineProps<{
    newsletterId: string;
}>();

const title = ref("");
const subtitle = ref("");

const editor = useEditor({
    content: "",
    extensions: [
        StarterKit,
        UnderlineExtension,
        ImageBlock,
        Link.extend({
            renderHTML({ HTMLAttributes }) {
                const { href, target, rel, ...rest } = HTMLAttributes;
                return ["a", rest, 0];
            },
        }).configure({ openOnClick: false }),
    ],
    editorProps: {
        attributes: { class: "editor-prose" },
    },
    onUpdate: (value) => {
        json.value.innerHTML = JSON.stringify(
            value.transaction.doc.toJSON(),
            null,
            5,
        );
        markDirty();
        debouncedSave();
    },
});

const json = ref();

const { save, debouncedSave, isDirty, markDirty } = useSave(async () => {
    const response = await fetch(`/json/${props.newsletterId}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            title: title.value,
            subtitle: subtitle.value,
            content: editor.value?.getJSON(),
        }),
    });
    return response.ok;
});

onMounted(async () => {
    const response = await fetch(`/json/${props.newsletterId}`);
    const data: { title: string; subtitle: string; content: Object } =
        await response.json();

    title.value = data.title;
    subtitle.value = data.subtitle;
    editor.value!.commands.setContent(data.content, { emitUpdate: false });

    watch([title, subtitle], () => {
        markDirty();
        debouncedSave();
    });
});

onKeyStroke("s", (event) => {
    if (event.ctrlKey || event.metaKey) {
        event.preventDefault();
        save();
    }
});

useEventListener(window, "beforeunload", (event) => {
    if (isDirty.value) event.preventDefault();
});
</script>

<template>
    <div class="editor">
        <div v-if="editor" class="editor-toolbar">
            <Toolbar :editor="editor" />
        </div>

        <div class="editor-meta">
            <input
                class="editor-title"
                type="text"
                name="title"
                placeholder="Newsletter Title"
                v-model="title"
            />
            <input
                class="editor-subtitle"
                type="text"
                name="subtitle"
                placeholder="Subtitle"
                v-model="subtitle"
            />
        </div>

        <editor-content :editor="editor" class="editor-content prose" />
    </div>

    <div hidden>
        <code ref="json"></code>
    </div>
</template>

<style scoped>
.editor {
    & > * {
        padding: 0 0.5em;
    }

    & .editor-meta {
        max-width: min(70ch, 100%);
        margin: 0 auto;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        padding: 1.5rem 1rem 0.5rem;

        & .editor-title,
        & .editor-subtitle {
            border: none;
            outline: none;
            background: transparent;
            width: 100%;
            font-family: inherit;
            color: inherit;
        }

        & .editor-title {
            font-size: 1.5rem;
            font-weight: 600;
        }

        & .editor-subtitle {
            font-size: 1rem;
            color: color-mix(in srgb, currentColor 60%, transparent);
        }
    }

    & .editor-toolbar {
        position: sticky;
        top: 4em;
        background-color: var(--color-surface);
        z-index: 10;
        /*border-bottom: 1px solid var(--color-border);*/
        background-color: var(--color-surface);
        padding: 0 8em;
    }

    & .editor-content {
        flex: 1;
        padding: 1rem;
    }
}
</style>

<style>
.editor-prose {
    outline: none;
    max-width: none;
    min-height: 80vh;

    display: grid;
    grid-template-columns: 1fr min(70ch, 100%) 1fr;

    & > * {
        grid-column: 2;
    }

    & > [data-width="full"] {
        grid-column: 1 / 4;
    }
}

.tiptap p.is-editor-empty:first-child::before {
    color: color-mix(in srgb, currentColor 40%, transparent);
    content: attr(data-placeholder);
    float: left;
    height: 0;
    pointer-events: none;
}
</style>
