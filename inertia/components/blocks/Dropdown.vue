<script setup lang="ts">
import { MenuRoot, MenuTrigger, MenuPositioner, MenuContent, MenuItem } from '@ark-ui/vue/menu';
import Button from '~/components/blocks/Button.vue';
import { type Component } from 'vue';

export interface DropdownItem {
    id: string;
    label: string;
    icon?: Component;
    href?: string;
    method?: 'get' | 'post' | 'put' | 'delete' | 'patch';
    action?: () => void;
}

const props = defineProps<{
    items: DropdownItem[];
    placement?: 'bottom-start' | 'bottom-end' | 'right-start' | 'top-start';
}>();

</script>

<template>
    <MenuRoot :positioning="{ placement: placement || 'bottom-start' }">
        <MenuTrigger as-child>
            <slot name="trigger" />
        </MenuTrigger>

        <MenuPositioner>
            <MenuContent
                class="min-w-[12rem] bg-white border border-black shadow-lg p-1.5 outline-none z-50 flex flex-col gap-1">
                <MenuItem v-for="item in items" :key="item.id" :value="item.id" as-child>
                <Button v-bind="item.href
                    ? {
                        as: item.href?.startsWith('http') ? 'anchor' : 'link', href: item.href, method: item.method
                            || 'get'
                    } : { as: 'button', onClick: item.action }" variant="ghost" size="sm"
                    class="w-full justify-start gap-2 data-[highlighted]:bg-surface-hover">
                    <component :is="item.icon" v-if="item.icon" class="w-4 h-4 text-subdued" />
                    <span>{{ item.label }}</span>
                </Button>
                </MenuItem>
            </MenuContent>
        </MenuPositioner>
    </MenuRoot>
</template>
