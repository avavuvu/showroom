<script setup lang="ts">
import { computed } from 'vue';

defineOptions({ inheritAttrs: false });

const sizes = ["sm", "md", "lg"] as const
export type Size = typeof sizes[number]

const props = defineProps<{
    src: string; // src = small Source
    mdSrc?: string, // curse that one email change that means we use src rather than source
    lgSrc?: string,
    alt?: string;
    width?: string | number;
    size?: Size
}>();

let src = props.src;

switch (props.size) {
    case "md":
        src = props.mdSrc || props.src
        break;
    case "lg":
        src = props.lgSrc || props.src
        break;
}

const sizeStyle = computed(() => {
    if (!props.size) return undefined;

    const s = typeof props.size === 'number' ? `${props.size}px` : props.size;
    return { width: s, height: s };
});
</script>

<template>
    <img :src :alt="props.alt || ''" class="inline-block select-none pointer-events-none object-contain"
        :style="sizeStyle" v-bind="$attrs" aria-hidden="true" />
</template>
