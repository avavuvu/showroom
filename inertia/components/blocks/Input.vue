<script setup lang="ts">
import { XIcon, } from 'lucide-vue-next';

const { type, placeholder, name, error } = defineProps<{
    type: "password" | "text",
    placeholder: string,
    name: string,
    error?: string | null
}>()

const model = defineModel()
</script>

<template>
    <div class="flex flex-col">
        <label :for="name" :arial-label="placeholder"></label>
        <input class="[.error]:border-error border border-black p-2" :class="{ error: error != null }" :placeholder
            :type="type" :name="name" :id="name" v-model="model">

        <span v-if="error != null" class="text-error inline-flex ">
            <XIcon />{{ error }}
        </span>
    </div>
</template>
