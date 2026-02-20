<script setup lang="ts">
import { Link, usePage } from '@inertiajs/vue3';
import { computed } from 'vue';
import Button from '~/components/blocks/Button.vue';
import Dropdown from '~/components/blocks/Dropdown.vue';
import { ChevronDown, Plus, User, LayoutDashboard, LogOut } from 'lucide-vue-next';
import { Scrap, Envelope } from '~/components/icons';
import type { SharedProps } from '@adonisjs/inertia/types';
import type UserType from '#models/user';
import Logo from '../icons/Logo.vue';

const page = usePage<SharedProps>();
const user = computed(() => page.props.user as UserType | undefined);

const profileUrl = computed(() => {
    if (!user.value) return '#';
    // Logic as per user feedback/codebase
    return `http://${user.value.username}.localtest.me:3333`;
});

const createOptions = [
    { id: 'newsletter', label: 'Newsletter', icon: Envelope, href: '/newsletters', method: 'post' as const },
    { id: 'scrap', label: 'Scrap', icon: Scrap, href: '/scrap', method: 'post' as const },
];

const profileOptions = [
    { id: 'view_profile', label: 'View Profile', icon: User, href: profileUrl.value },
    { id: 'logout', label: 'Log Out', icon: LogOut, href: '/logout' },
]

</script>

<template>
    <aside class="h-screen w-64 bg-white border-r border-border flex flex-col fixed left-0 top-0 z-50">
        <!-- Logo -->
        <div class="p-6">
            <Link href="/" class="block">
                <Logo class="h-8 w-auto" size="sm" />
            </Link>
        </div>

        <!-- Create Button -->
        <div class="px-4 mb-6">
            <Dropdown :items="createOptions" placement="bottom-start">
                <template #trigger>
                    <Button variant="solid" size="md" class="w-full justify-between gap-2">
                        <div class="flex items-center gap-2">
                            <Plus class="w-4 h-4" />
                            Create
                        </div>
                        <ChevronDown class="w-4 h-4 opacity-70" />
                    </Button>
                </template>
            </Dropdown>
        </div>

        <!-- Navigation -->
        <nav class="flex-1 px-4 space-y-1">
            <Button as="link" href="/dashboard" variant="outline" class="w-full justify-start gap-3"
                :class="{ '  bg-surface-hover! font-bold!': $page.url.startsWith('/dashboard') }">
                <LayoutDashboard class="w-4 h-4 text-subdued" />
                Dashboard
            </Button>

            <Button as="link" href="/profile" variant="outline" class="w-full justify-start gap-3"
                :class="{ ' bg-surface-hover! font-bold!': $page.url.startsWith('/profile') }">
                <User class="w-4 h-4 text-subdued" />
                Profile
            </Button>
        </nav>

        <!-- Profile Dropdown -->
        <div class="p-4 border-t border-border" v-if="user">
            <Dropdown :items="profileOptions" placement="top-start">
                <template #trigger>
                    <Button variant="ghost" class="w-full justify-start gap-0 h-auto">
                        <div class="w-8 h-8 flex items-center justify-center shrink-0 ">
                            <img v-if="user.profileImageUrl" :src="user.profileImageUrl">
                            <User v-else class="w-4 h-4 text-subdued" />
                        </div>

                        <div class="px-2 border-l border-border flex-1 min-w-0 text-left">
                            <p class="text-sm font-medium text-black truncate">{{ user.fullName }}</p>
                            <p class="text-xs text-subdued truncate">{{ user.username }}.showroom.you</p>
                        </div>
                        <ChevronDown class="w-4 h-4" />
                    </Button>
                </template>
            </Dropdown>
        </div>
    </aside>
</template>
