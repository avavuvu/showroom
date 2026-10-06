<script setup lang="ts">
import { onMounted, ref, useTemplateRef } from "vue";
import { CheckIcon, Trash } from "lucide-vue-next";

const props = defineProps<{ initialHref: string; autofocus?: boolean }>();
const emit = defineEmits<{ save: [href: string]; remove: []; close: [] }>();

const linkUrl = ref(props.initialHref);
const input = useTemplateRef("input");

const normalizeUrl = (url: string) => {
    const trimmed = url.trim();
    if (!trimmed) return "";
    if (/^[a-z][a-z0-9+.-]*:/i.test(trimmed)) return trimmed;
    if (/^\/|^#/.test(trimmed)) return trimmed;
    if (/^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(trimmed)) return `mailto:${trimmed}`;
    return `https://${trimmed}`;
};

const save = () => {
    if (!linkUrl.value.trim()) {
        emit("remove");
        return;
    }
    emit("save", normalizeUrl(linkUrl.value));
};

const onKeydown = (event: KeyboardEvent) => {
    if (event.key === "Escape") {
        event.preventDefault();
        emit("close");
    } else if (event.key === "Enter") {
        event.preventDefault();
        save();
    }
};

const onInputMousedown = (event: MouseEvent) => {
    if (!event.metaKey && !event.ctrlKey) return;
    if (!linkUrl.value) return;
    event.preventDefault();
    window.open(linkUrl.value, "_blank", "noopener,noreferrer");
};

onMounted(() => {
    if (!props.autofocus) return;
    requestAnimationFrame(() => {
        input.value?.focus();
        input.value?.select();
    });
});
</script>

<template>
    <div class="link-menu" contenteditable="false">
        <input
            ref="input"
            class="link-input"
            v-model="linkUrl"
            type="text"
            placeholder="Enter URL..."
            @keydown="onKeydown"
            @mousedown="onInputMousedown"
        />
        <button class="link-btn" type="button" title="Save" @mousedown.prevent="save">
            <CheckIcon />
        </button>
        <button
            class="link-btn"
            type="button"
            title="Remove link"
            @mousedown.prevent="emit('remove')"
        >
            <Trash />
        </button>
    </div>
</template>


