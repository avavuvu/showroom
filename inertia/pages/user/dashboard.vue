<script setup lang="ts">
import type User from '#models/user'
import type Newsletter from '#models/newsletter'
import { Deferred } from '@inertiajs/vue3'
import Display from '~/components/Newsletters.vue/Display.vue';
import type Scrap from '#models/scrap';
import ScrapDisplay from '~/components/Scrap/ScrapDisplay.vue';

defineProps<{
  user: User,
  newsletters?: Newsletter[],
  scraps?: Scrap[]
}>()


</script>

<template>
  <h1>{{ user.username }}'s Dashboard</h1>

  <Deferred data="newsletters">
    <template #fallback>
      <div>Loading...</div>
    </template>

    <Display :newsletters="newsletters!" />
  </Deferred>

  <Deferred data="scraps">
    <template #fallback>
      <div>Loading...</div>
    </template>

    <ScrapDisplay :scraps="scraps!" />
  </Deferred>

</template>
