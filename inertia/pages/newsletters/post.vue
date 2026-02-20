<script setup lang="ts">
import type Newsletter from '#models/newsletter';
import Comments from '~/components/comments/Comments.vue';
import UserProfileLayout from '~/layouts/UserProfileLayout.vue';

defineProps<{ newsletter: Newsletter }>()
</script>

<template>
    comments: {{ JSON.stringify(newsletter.comments) }}

    <UserProfileLayout>
        <div class="max-w-3xl mx-auto">
            <div>
                <h1 class="font-title text-4xl font-bold">
                    {{ newsletter.title }}
                </h1>
                <span class="font-title italic" v-if="newsletter.subtitle">
                    {{ newsletter.subtitle }}
                </span>

            </div>
        </div>
        <article class="grid grid-cols-[1fr_70ch_1fr] article-content" v-html="newsletter.content"></article>

        <div class="max-w-2xl mx-auto">
            <Comments :comments="newsletter.comments" />

        </div>
    </UserProfileLayout>

</template>

<style scoped>
.article-content>* {
    grid-column-start: 2;
}
</style>