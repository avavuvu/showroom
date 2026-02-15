<script setup lang="ts">
import type User from '#models/user'
import type Newsletter from '#models/newsletter'
import { Deferred } from '@inertiajs/vue3'
import Display from '~/components/Newsletters.vue/Display.vue';
import type Scrap from '#models/scrap';
import ScrapDisplay from '~/components/Scrap/ScrapDisplay.vue';
import { Link } from '@inertiajs/vue3';
import AuthenticatedLayout from '~/layouts/AuthenticatedLayout.vue';

defineProps<{
  user: User,
  newsletters?: Newsletter[],
  scraps?: Scrap[]
}>()
</script>

<template>
  <AuthenticatedLayout>
    <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <h1 class="font-title font-bold text-2xl mb-6">{{ user.username }}<span class="font-normal">.showroom.you</span></h1>

      <Link href="/logout" class="text-sm text-gray-500 hover:text-black mb-8 block">Logout</Link>

      <Deferred data="newsletters">
        <template #fallback>
          <div class="py-4">Loading newsletters...</div>
        </template>

        <Display :newsletters="newsletters!" />
      </Deferred>

      <Deferred data="scraps">
        <template #fallback>
          <div class="py-4">Loading scraps...</div>
        </template>

        <ScrapDisplay :scraps="scraps!" />
      </Deferred>
    </div>
  </AuthenticatedLayout>
</template>
