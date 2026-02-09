<script setup lang="ts">
import type Newsletter from '#models/newsletter';
import { Link } from '@inertiajs/vue3'

const { newsletters } = defineProps<{
    newsletters: Newsletter[]
}>()

const [sentNewsletters, unsentNewsletters] = newsletters.reduce((accumulator, newsletter) => {
    if (newsletter.sent) {
        accumulator[0].push(newsletter)
    } else {
        accumulator[1].push(newsletter)
    }
    return accumulator
}, [[], []] as [Newsletter[], Newsletter[]])


</script>

<template>
    <Link href="/newsletters" method="post" as="button" class="block mb-4">create new newsletter</Link>

    <h2>Edit Drafts</h2>
    <div class="grid gap-4">
        <div v-for="newsletter in unsentNewsletters" :key="newsletter.id" class="p-4">
            <Link :href="`/publish/${newsletter.id}`" class="text-xl font-bold hover:underline">
                {{ newsletter.title || 'Untitled Newsletter' }}
            </Link>
            <div class="text-gray-500 text-sm">
                {{ new Date(newsletter.createdAt.toString()).toLocaleDateString() }}
            </div>
        </div>
        <div v-if="unsentNewsletters.length === 0" class="">
            You haven't created any newsletters yet.
        </div>
    </div>
</template>