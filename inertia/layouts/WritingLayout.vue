<script setup lang="ts">
import logoIcon from "~/icons/logo/img.png";
import { Link, usePage } from '@inertiajs/vue3';
import Button from "~/components/blocks/Button.vue";
import { ChevronLeft } from 'lucide-vue-next';

</script>

<template>
    <div class="min-h-screen">
        <header class="flex justify-between items-center px-2 h-12 bg-white border-b border-border sticky top-0 z-50">
            <div class="flex">
                <div class="">
                    <Link href="/" class="block">
                        <img class="h-8 w-auto" :src="logoIcon" alt="Showroom Logo" />
                    </Link>
                </div>
                <nav class="">
                    <Button as="link" href="/dashboard" variant="ghost" class="">
                        <ChevronLeft class="w-4 h-4 " />
                    </Button>
                </nav>


            </div>

            <div>
                <slot name="right-most"></slot>
            </div>


        </header>

        <main class="">
            <slot />
        </main>
    </div>
</template>
