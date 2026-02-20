<script setup lang="ts">
import Shicon, { type Size } from './Shicon.vue';
import src from '~/icons/logo/pear.png';

defineProps<{
    width?: string | number;
    size?: Size
}>();
</script>

<template>
    <Shicon :src alt="Logo" :width :size />
</template>
