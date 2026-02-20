<script setup lang="ts">
import logoIcon from "~/icons/logo/img.png"
import { usePage } from "@inertiajs/vue3"
import Button from "~/components/blocks/Button.vue"
import type { SharedProps } from '@adonisjs/inertia/types'
import SubscribeDialog from "~/components/newsletters/SubscribeDialog.vue"
import Logo from "~/components/icons/Logo.vue"

const { user, subdomainUser } = usePage<SharedProps>().props

const isCurrentUser = user
    && subdomainUser
    && user.username === subdomainUser.username

</script>

<template>
    <header class="sticky top-0 z-10 w-full flex justify-between bg-white border-b">
        <div class="px-2 flex items-center">
            <Logo class="h-8" />
        </div>
        <div class="flex justify-end items-end">


            <template v-if="isCurrentUser">
                <Button as="anchor" href="/dashboard" variant="ghost">Dashboard</Button>
            </template>
            <template v-else>
                <SubscribeDialog :username="subdomainUser!.username!" />
            </template>
        </div>

    </header>

    <main>
        <slot />
    </main>
</template>
