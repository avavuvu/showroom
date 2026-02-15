<script setup lang="ts">
// const { title } = defineProps<{ title: string }>()
</script>

<template>
    <div class="p-2 border border-black">
        <slot name="title"></slot>
        <slot />
    </div>
</template>