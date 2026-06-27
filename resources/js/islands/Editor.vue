<script setup lang="ts">
import { useEditor, EditorContent } from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";
import UnderlineExtension from "@tiptap/extension-underline";
import { onMounted, onUnmounted, ref, watch } from "vue";
import Toolbar from "../editor/Toolbar.vue";
import { useSave } from "../editor/useSave";
import { onKeyStroke } from "@vueuse/core";

const props = defineProps<{
    newsletterId: string;
}>();

const title = ref("");
const subtitle = ref("");

const editor = useEditor({
    content: "",
    extensions: [StarterKit, UnderlineExtension],
    editorProps: {
        attributes: {
            class: "outline-none prose max-w-none",
        },
    },
    onUpdate: (value) => {
        json.value.innerHTML = JSON.stringify(
            value.transaction.doc.toJSON(),
            null,
            5,
        );
        isDirty.value = true;
        debouncedSave();
    },
});

const json = ref();

const { save, debouncedSave, isSaving, isDirty } = useSave(async () => {
    const newsletterUpdate = {
        title: title.value,
        subtitle: subtitle.value,
        content: editor.value?.getJSON(),
    };

    await fetch(`/json/${props.newsletterId}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newsletterUpdate),
    });
});

onMounted(async () => {
    const response = await fetch(`/json/${props.newsletterId}`);
    const json: {
        title: string;
        subtitle: string;
        content: Object;
    } = await response.json();

    title.value = json.title;
    subtitle.value = json.subtitle;
    editor.value!.commands.setContent(json.content, {
        emitUpdate: false,
    });

    watch([title, subtitle], () => {
        isDirty.value = true;
        debouncedSave();
    });
});

onKeyStroke("s", (event) => {
    if (event.ctrlKey || event.metaKey) {
        event.preventDefault();
        save();
    }
});

onUnmounted(() => {
    editor.value?.destroy();
});
</script>

<template>
    <div class="">
        <input
            type="text"
            name="title"
            class=""
            placeholder="Newsletter Title"
            v-model="title"
        />
        <input
            type="text"
            name="subtitle"
            class=""
            placeholder="Subtitle"
            v-model="subtitle"
        />
        <div>
            <template v-if="isDirty"> Saving... </template>
            <template v-else> Saved </template>
        </div>

        <div v-if="editor" class="border-b border-border bg-surface">
            <Toolbar :editor="editor" />
        </div>
        <editor-content :editor="editor" class="min-h-64 p-4" />
    </div>

    <div>
        <code ref="json"> </code>
    </div>
</template>

<style>
.tiptap p.is-editor-empty:first-child::before {
    color: #9ca3af;
    content: attr(data-placeholder);
    float: left;
    height: 0;
    pointer-events: none;
}
</style>
