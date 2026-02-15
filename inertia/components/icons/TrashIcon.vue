<script setup lang="ts">
import Shicon from './Shicon.vue';
import iconSrc from '~/icons/bin/trash.png';

defineProps<{
    size?: string | number;
}>();
</script>

<template>
    <Shicon :src="iconSrc" alt="Trash" :size="size" />
</template>
