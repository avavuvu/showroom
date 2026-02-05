<script setup lang="ts">
import { MilkdownProvider } from "@milkdown/vue";
import MilkdownEditor from "./MilkdownEditor.vue"
import { ProsemirrorAdapterProvider } from "@prosemirror-adapter/vue"
import type Newsletter from "#models/newsletter";

defineProps<{
  newsletter: Newsletter
}>()

</script>


<template>
  <MilkdownProvider>
    <ProsemirrorAdapterProvider>
      <MilkdownEditor :newsletter="newsletter" />
    </ProsemirrorAdapterProvider>
  </MilkdownProvider>
</template>
