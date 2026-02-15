<script setup lang="ts">
import { computed } from 'vue';

defineOptions({ inheritAttrs: false });

const props = defineProps<{
    src: string;
    alt?: string;
    size?: string | number;
}>();

const sizeStyle = computed(() => {
    if (!props.size) return undefined;
    const s = typeof props.size === 'number' ? `${props.size}px` : props.size;
    return { width: s, height: s };
});
</script>

<template>
    <img :src="props.src" :alt="props.alt || ''" class="inline-block select-none pointer-events-none object-contain"
        :style="sizeStyle" v-bind="$attrs" aria-hidden="true" />
</template>
