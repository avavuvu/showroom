<script setup lang="ts">
import AuthenticatedSidebar from '~/components/layout/AuthenticatedSidebar.vue';
</script>

<template>
    <div class="min-h-screen flex">
        <AuthenticatedSidebar />

        <main class="flex-1 ml-64 min-w-0">
            <slot />
        </main>
    </div>
</template>
