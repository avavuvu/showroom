<script setup lang="ts">
import Shicon from './Shicon.vue';
import src from '~/icons/ui/scrap.png';

defineProps<{
    size?: string | number;
}>();
</script>

<template>
    <Shicon :src alt="Scrap" :size="size" />
</template>
