<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { CheckIcon, Trash, ExternalLink } from "lucide-vue-next";
import { getMarkRange, type Editor } from "@tiptap/vue-3";

const props = defineProps<{ editor: Editor }>();

const menu = ref<HTMLElement | null>(null);
const open = ref(false);
const linkUrl = ref("");
const linkDisplay = ref("");
const linkMenuPosition = ref({ top: "0px", left: "0px" });

const MENU_WIDTH = 240;
const MARGIN = 8;

const setLink = () => {
    const { view, state } = props.editor;
    const { from, to, $from } = state.selection;
    let start = view.coordsAtPos(from);
    let end = view.coordsAtPos(to);
    linkDisplay.value = state.doc.textBetween(from, to, "");

    const range = getMarkRange($from, state.schema.marks.link);
    if (range) {
        linkDisplay.value = state.doc.textBetween(range.from, range.to, "");
        start = view.coordsAtPos(range.from);
        end = view.coordsAtPos(range.to);
    }

    const rawLeft = (start.left + end.left) / 2;
    const clampedLeft = Math.max(
        MENU_WIDTH / 2 + MARGIN,
        Math.min(window.innerWidth - MENU_WIDTH / 2 - MARGIN, rawLeft),
    );

    linkMenuPosition.value = {
        top: `${end.bottom + 10}px`,
        left: `${clampedLeft}px`,
    };

    linkUrl.value = props.editor.getAttributes("link").href || "";
    open.value = true;
};

const saveLink = () => {
    if (!linkUrl.value) {
        removeLink();
        return;
    }

    const normalizedUrl = normalizeUrl(linkUrl.value);
    const { state } = props.editor;
    const { $from } = state.selection;
    const range = getMarkRange($from, state.schema.marks.link);
    const { from, to } = range ?? state.selection;

    props.editor
        .chain()
        .focus()
        .deleteRange({ from, to })
        .insertContent(linkDisplay.value)
        .run();

    const newTo = from + linkDisplay.value.length;
    props.editor
        .chain()
        .focus()
        .setTextSelection({ from, to: newTo })
        .setLink({ href: normalizedUrl })
        .run();

    closeLinkMenu();
};

const removeLink = () => {
    props.editor.chain().focus().extendMarkRange("link").unsetLink().run();
    closeLinkMenu();
};

const closeLinkMenu = () => {
    open.value = false;
    linkUrl.value = "";
};

const normalizeUrl = (url: string) => {
    if (!url?.trim()) return "";
    let normalized = url.trim();
    if (!/^https?:\/\//i.test(normalized)) normalized = "https://" + normalized;
    try {
        return new URL(normalized).href;
    } catch {
        return url;
    }
};

defineExpose({ setLink });

const onClickOutside = (e: MouseEvent) => {
    if (menu.value && !menu.value.contains(e.target as Node)) closeLinkMenu();
};
const onKeydown = (e: KeyboardEvent) => {
    if (e.key === "Escape") closeLinkMenu();
};

onMounted(() => {
    document.addEventListener("mousedown", onClickOutside);
    document.addEventListener("keydown", onKeydown);
});
onUnmounted(() => {
    document.removeEventListener("mousedown", onClickOutside);
    document.removeEventListener("keydown", onKeydown);
});

props.editor.on("selectionUpdate", ({ transaction }) => {
    const linkMark = transaction.doc
        .nodeAt(transaction.selection.from)
        ?.marks.find((m) => m.type.name === "link");
    if (linkMark) setLink();
    else if (open.value) closeLinkMenu();
});
</script>

<template>
    <div
        v-if="open"
        ref="menu"
        class="link-menu"
        :style="{ ...linkMenuPosition, transform: 'translateX(-50%)' }"
    >
        <input
            class="link-input"
            v-model="linkDisplay"
            type="text"
            placeholder="Display as..."
            @keydown.enter="saveLink"
            @keydown.escape="closeLinkMenu"
        />
        <div class="link-url-row">
            <input
                class="link-input link-input--url"
                v-model="linkUrl"
                type="text"
                placeholder="Enter URL..."
                @keydown.enter="saveLink"
                @keydown.escape="closeLinkMenu"
            />
            <a :href="linkUrl" target="_blank" class="link-external">
                <ExternalLink />
            </a>
        </div>
        <div class="link-actions">
            <button class="link-btn" @click="saveLink" title="Save">
                <CheckIcon />
            </button>
            <button class="link-btn" @click="removeLink" title="Remove link">
                <Trash />
            </button>
        </div>
    </div>
</template>

<style scoped>
.link-menu {
    position: fixed;
    z-index: 100;
    display: grid;
    gap: 0.25rem;
    padding: 0.5rem;
    background-color: var(--color-surface);
    border: 1px solid var(--color-border);
    box-shadow: 0 4px 12px rgb(0 0 0 / 0.1);
    width: v-bind("`${MENU_WIDTH}px`");

    & .link-input {
        outline: none;
        border: 1px solid var(--color-border);
        padding: 0.25rem 0.5rem;
        background: transparent;
        color: inherit;
        font-family: inherit;
        font-size: 0.875rem;
        width: 100%;

        &.link-input--url {
            text-decoration: underline;
        }
    }

    & .link-url-row {
        display: flex;
        align-items: center;
        border: 1px solid var(--color-border);
        padding: 0 0.5rem;

        & .link-input {
            border: none;
            padding: 0.25rem 0;
        }

        & .link-external {
            display: flex;
            align-items: center;
            color: inherit;
            opacity: 0.6;

            &:hover {
                opacity: 1;
            }
            & svg {
                width: 1rem;
                height: 1rem;
            }
        }
    }

    & .link-actions {
        display: flex;
        justify-content: flex-end;
        gap: 0.25rem;

        & .link-btn {
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 0.25rem;
            background: transparent;
            border: none;
            cursor: pointer;
            color: inherit;
            border-radius: 4px;

            &:hover {
                background-color: var(--color-surface-hover);
            }
            & svg {
                width: 1rem;
                height: 1rem;
            }
        }
    }
}
</style>
