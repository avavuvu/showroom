<script setup lang="ts">
import Shicon from './Shicon.vue';
import src from '~/icons/ui/envelope.png';

defineProps<{
    size?: string | number;
}>();
</script>

<template>
    <Shicon :src alt="Envelope" :size="size" />
</template>
