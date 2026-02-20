<script setup lang="ts">
import type User from '#models/user'
import type Newsletter from '#models/newsletter'
import { Deferred } from '@inertiajs/vue3'
import UserProfileLayout from '~/layouts/UserProfileLayout.vue';

defineProps<{
    newsletters: Newsletter[]
}>()


</script>

<template>
    <UserProfileLayout>
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

    </UserProfileLayout>
</template>
