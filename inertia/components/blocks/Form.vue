<script setup lang="ts">
import { XIcon } from 'lucide-vue-next';

const { onSubmit, error } = defineProps<{
    onSubmit: (event: Event) => void,
    error?: string | null
}>()
</script>

<template>
    <form method="POST" @submit="onSubmit" class="flex flex-col gap-4">
        <span v-if="error != null" class="text-error inline-flex ">
            <XIcon />{{ error }}
        </span>

        <slot />
    </form>
</template>