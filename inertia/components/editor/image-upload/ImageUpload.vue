<script setup lang="ts">
import { ref } from 'vue';

defineProps

const imageRef = ref<HTMLImageElement>()

const width = ref<"normal" | "full">("normal")

</script>

<template>
    <img ref="imageRef">
</template>