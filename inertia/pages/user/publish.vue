<script setup lang="ts">
import MilkdownEditorWrapper from '~/components/MilkdownEditorWrapper.vue';
import type Newsletter from '#models/newsletter';

defineProps<{
  newsletter: Newsletter
}>()
</script>

<template>
  <MilkdownEditorWrapper :newsletter="newsletter" />
</template>
