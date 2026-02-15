<script setup lang="ts">
import type Scrap from '#models/scrap';
import { Link } from '@inertiajs/vue3'
import Card from '../blocks/Card.vue';
import Button from '../blocks/Button.vue';

const { scraps } = defineProps<{
    scraps: Scrap[]
}>()

const formattedScraps = scraps.filter(scrap => scrap.isSuccessful).map(scrap => ({
    ...scrap,
    content: scrap.content.replace(/<[^>]*>/g, '') + "...",
}))

</script>

<template>
    <h2>Scraps</h2>
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
        <Card class="w-48 aspect-square" v-for="scrap in formattedScraps" :key="scrap.id">
            <template #title>
                <Link :href="`/scrap/${scrap.id}`" class="line-clamp-4 hover:underline">
                    {{ scrap.content }}
                </Link>
            </template>

            <div class="text-gray-500 text-sm text-right">
                {{ new Date(scrap.createdAt.toString()).toLocaleDateString() }}
            </div>
        </Card>
        <Button href="/scrap" method="post" as="link" color="secondary"
            class="w-48 aspect-square text-subdued underline">
            <span v-if="formattedScraps.length === 0" class="">
                You haven't written on any scrap paper yet.
            </span>
            Start a new Scrap
        </Button>




    </div>
</template>