<script setup lang="ts">
import type User from '#models/user'
import type Newsletter from '#models/newsletter'
import { Deferred } from '@inertiajs/vue3'

defineProps<{
    newsletters: Newsletter[]
}>()


</script>

<template>
    <Deferred data="newsletters">
        <template #fallback>
            <div>Loading...</div>
        </template>

        <ul>
            <li v-for="newsletter in newsletters" :key="newsletter.id">
                {{ newsletter.title }}
            </li>
        </ul>
    </Deferred>
</template>
