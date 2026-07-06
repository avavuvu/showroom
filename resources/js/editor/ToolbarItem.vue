<script setup lang="ts">
import { type Component } from "vue";

defineProps<{
    isActive?: boolean;
    disabled?: boolean;
    icon?: Component;
    title?: string;
}>();
</script>

<template>
    <button
        type="button"
        :disabled="disabled"
        class="toolbar-btn"
        :class="{ 'is-active': isActive, 'is-disabled': disabled }"
        :title="title"
    >
        <component :is="icon" v-if="icon" class="toolbar-icon" />
        <slot v-else />
    </button>
</template>

<style scoped>
.toolbar-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem;
    border: none;
    background: transparent;
    cursor: pointer;
    border-radius: 4px;
    color: inherit;
    transition: background-color 150ms;

    &:hover {
        background-color: var(--color-surface-hover);
    }

    &.is-active {
        background-color: var(--color-surface-hover);
        color: var(--color-primary);
    }

    &.is-disabled,
    &:disabled {
        opacity: 0.5;
        cursor: not-allowed;
        pointer-events: none;
    }

    & .toolbar-icon {
        width: 1rem;
        height: 1rem;
    }
}
</style>
