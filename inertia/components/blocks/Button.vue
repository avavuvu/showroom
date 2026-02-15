<script setup lang="ts">
import { Link } from '@inertiajs/vue3';
import { computed } from 'vue';

const {
    disabled,
    type,
    onClick,
    as = "button",
    href,
    variant = "solid",
    size = "md",
    method = "get",
    animate = "slide"
} = defineProps<{
    disabled?: boolean
    type?: 'submit' | 'button' | 'reset',
    onClick?: (event: Event) => void,
    as?: "anchor" | "button" | "link",
    href?: string,
    variant?: "solid" | "outline" | "ghost",
    size?: "sm" | "md" | "lg",
    method?: "post" | "get" | "put" | "delete" | "patch"
    animate?: "none" | "slide"
}>()

let componentType;
switch (as) {
    case "anchor":
        componentType = "a"
        break;
    case "button":
        componentType = "button"
        break;
    case "link":
        componentType = Link
}

const classes = computed(() => {
    return [
        // Base styles
        'sr-button relative flex items-center justify-center cursor-pointer font-medium transition-colors duration-200',

        // Variants
        variant === 'solid' && 'bg-black text-white border border-black hover:opacity-80',
        variant === 'outline' && 'bg-transparent text-black border border-black hover:bg-surface-subtle',
        variant === 'ghost' && 'bg-transparent text-black border-transparent hover:bg-surface-hover',

        // Sizes
        size === 'sm' && 'px-3 py-1.5 text-sm',
        size === 'md' && 'px-4 py-2 text-base',
        size === 'lg' && 'px-6 py-3 text-lg',

        // State
        disabled && 'opacity-50 cursor-not-allowed',
        animate === 'slide' && variant === 'solid' && 'animate-slide overflow-hidden',
    ]
})
</script>

<template>
    <component v-bind:is="componentType" :class="classes" @click="onClick" :type="type" :href :method>
        <span class="relative z-10 flex items-center gap-2">
            <slot />
        </span>
    </component>
</template>

<style scoped>
.sr-button.animate-slide::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: var(--color-primary, #333);
    /* Fallback color */
    transform: scaleX(0);
    transform-origin: bottom left;
    transition: transform 0.3s ease-out;
    z-index: 0;
}

.sr-button.animate-slide:hover::after {
    transform: scaleX(1);
    transform-origin: bottom left;
}
</style>