<script setup lang="ts">
import { ref, onMounted, onUnmounted, type Component } from "vue";
import { ChevronDown } from "lucide-vue-next";

export interface ToolbarDropdownItem {
    id: string;
    label: string;
    icon?: Component;
    action: () => void;
    isActive?: boolean;
}

defineProps<{
    items: ToolbarDropdownItem[];
    label?: string;
    icon?: Component;
    isActive?: boolean;
}>();

const open = ref(false);
const root = ref<HTMLElement | null>(null);

const toggle = () => (open.value = !open.value);

const select = (item: ToolbarDropdownItem) => {
    item.action();
    open.value = false;
};

const onClickOutside = (e: MouseEvent) => {
    if (root.value && !root.value.contains(e.target as Node)) {
        open.value = false;
    }
};

const onKeydown = (e: KeyboardEvent) => {
    if (e.key === "Escape") open.value = false;
};

onMounted(() => {
    document.addEventListener("mousedown", onClickOutside);
    document.addEventListener("keydown", onKeydown);
});

onUnmounted(() => {
    document.removeEventListener("mousedown", onClickOutside);
    document.removeEventListener("keydown", onKeydown);
});
</script>

<template>
    <div ref="root" class="dropdown">
        <button
            type="button"
            class="dropdown-trigger"
            :class="{ 'is-active': isActive || open }"
            @click="toggle"
        >
            <component :is="icon" v-if="icon" class="dropdown-icon" />
            <span v-if="label">{{ label }}</span>
            <ChevronDown class="dropdown-chevron" />
        </button>

        <div v-if="open" class="dropdown-menu">
            <button
                v-for="item in items"
                :key="item.id"
                type="button"
                class="dropdown-item"
                :class="{ 'is-active': item.isActive }"
                @click="select(item)"
            >
                <component
                    :is="item.icon"
                    v-if="item.icon"
                    class="dropdown-icon"
                />
                <span>{{ item.label }}</span>
            </button>
        </div>
    </div>
</template>

<style scoped>
.dropdown {
    position: relative;

    & .dropdown-trigger {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.25rem;
        padding: 0.5rem;
        border: none;
        background: transparent;
        cursor: pointer;
        font-size: 0.875rem;
        font-weight: 500;
        color: inherit;
        border-radius: 4px;
        transition: background-color 150ms;
        white-space: nowrap;

        &:hover,
        &.is-active {
            background-color: var(--color-surface-hover);
        }

        & .dropdown-icon {
            width: 1rem;
            height: 1rem;
        }
        & .dropdown-chevron {
            width: 0.75rem;
            height: 0.75rem;
            opacity: 0.5;
        }
    }

    & .dropdown-menu {
        position: absolute;
        top: 100%;
        left: 0;
        min-width: 10rem;
        background-color: var(--color-surface);
        border: 1px solid var(--color-border);
        box-shadow: 0 4px 12px rgb(0 0 0 / 0.1);
        padding: 0.25rem;
        z-index: 50;
        display: flex;
        flex-direction: column;
        gap: 0.125rem;

        & .dropdown-item {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.375rem 0.5rem;
            font-size: 0.875rem;
            background: transparent;
            border: none;
            cursor: pointer;
            width: 100%;
            text-align: left;
            color: inherit;
            border-radius: 4px;
            transition: background-color 150ms;

            &:hover,
            &.is-active {
                background-color: var(--color-surface-hover);
            }

            & .dropdown-icon {
                width: 1rem;
                height: 1rem;
            }
        }
    }
}
</style>
