<script setup lang="ts">
import type Scrap from '#models/scrap';
import { Link } from '@inertiajs/vue3'

const { scraps } = defineProps<{
    scraps: Scrap[]
}>()

const formattedScraps = scraps.filter(scrap => scrap.isSuccessful).map(scrap => ({
    ...scrap,
    content: scrap.content.split(" ").slice(0, 50).join(" ") + "...",
}))

</script>

<template>
    <h2>Scraps</h2>
    <div class="grid gap-4 grid-cols-4">
        <div v-for="scrap in formattedScraps" :key="scrap.id" class="p-4 max-w-3xl aspect-square">
            <Link :href="`/scrap/${scrap.id}`" class="text-xl font-bold hover:underline">
                {{ scrap.content }}
            </Link>
            <div class="text-gray-500 text-sm">
                {{ new Date(scrap.createdAt.toString()).toLocaleDateString() }}
            </div>
        </div>

        <Link href="/scrap" method="post" as="button" class="border cursor-pointer ">
            Start a new Scrap
        </Link>

        <div v-if="formattedScraps.length === 0" class="">
            You haven't written on any scrap paper yet.
        </div>
    </div>
</template>