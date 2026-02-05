<script setup lang="ts">
import type User from '#models/user'
import type Newsletter from '#models/newsletter'
import { Deferred } from '@inertiajs/vue3'
import Display from '~/components/Newsletters.vue/Display.vue';

defineProps<{
  user: User,
  newsletters: Newsletter[]
}>()


</script>

<template>
  <h1>{{ user.username }}'s Dashboard</h1>

  <Deferred data="newsletters">
    <template #fallback>
      <div>Loading...</div>
    </template>

    <Display :newsletters="newsletters" />
  </Deferred>

</template>
