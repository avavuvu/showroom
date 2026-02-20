<script setup lang="ts">
import type Newsletter from '#models/newsletter';
import { Link } from '@inertiajs/vue3'
import Card from '../blocks/Card.vue';
import Button from '../blocks/Button.vue';

const { newsletters } = defineProps<{
    newsletters: {
        sent: Newsletter[],
        unsent: Newsletter[],
    }
}>()

</script>

<template>
    <div>
        <Card class="w-full" v-for="newsletter in newsletters.sent" :key="newsletter.id">
            <h3>{{ newsletter.title }}</h3>
        </Card>
    </div>


    <h2>Edit Drafts</h2>
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
        <Card class="w-48 aspect-square" v-for="newsletter in newsletters.unsent" :key="newsletter.id">
            <template #title>
                <Link :href="`/publish/${newsletter.id}`" class=" hover:underline">
                    <h3 class="font-bold">
                        {{ newsletter.title || 'Untitled Newsletter' }}
                    </h3>
                </Link>
            </template>

            <div class="text-gray-500 text-sm text-right">
                {{ new Date(newsletter.createdAt.toString()).toLocaleDateString() }}
            </div>

            <div class="line-clamp-4">
                {{ newsletter.content.replace(/<[^>]*>/g, '') }}
            </div>

        </Card>
        <Button class="w-48 aspect-square text-subdued underline" as="link" href="/newsletters" method="post"
            color="secondary">
            Create new newsletter
        </Button>

        <div>
        </div>
        <div v-if="newsletters.unsent.length === 0" class="">
            You haven't created any newsletters yet.
        </div>
    </div>
</template>