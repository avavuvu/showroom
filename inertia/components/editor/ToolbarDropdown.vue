<script setup lang="ts">
import { MenuRoot, MenuTrigger, MenuPositioner, MenuContent, MenuItem } from '@ark-ui/vue/menu';
import { ChevronDown } from 'lucide-vue-next';
import { type Component } from 'vue';

export interface ToolbarDropdownItem {
    id: string;
    label: string;
    icon?: Component;
    action: () => void;
    isActive?: boolean;
}

defineProps<{
    items: ToolbarDropdownItem[];
    label?: string;
    icon?: Component;
    isActive?: boolean;
}>();
</script>

<template>
    <MenuRoot>
        <MenuTrigger as-child>
            <button
                class="flex justify-between items-center gap-1 p-2 hover:bg-surface-hover transition-colors text-sm font-medium text-black"
                :class="{ 'bg-surface-hover': isActive }">
                <component :is="icon" v-if="icon" class="w-4 h-4" />
                <span v-if="label">{{ label }}</span>
                <ChevronDown class="w-3 h-3 opacity-50" />
            </button>
        </MenuTrigger>

        <MenuPositioner>
            <MenuContent
                class="min-w-[10rem] bg-white border border-border shadow-lg p-1 outline-none z-50 flex flex-col gap-0.5">
                <MenuItem v-for="item in items" :key="item.id" :value="item.id" as-child>
                <button @click="item.action"
                    class="flex items-center gap-2 px-2 py-1.5 text-sm text-black hover:bg-surface-hover w-full text-left"
                    :class="{ 'bg-surface-hover': item.isActive }">
                    <component :is="item.icon" v-if="item.icon" class="w-4 h-4" />
                    <span>{{ item.label }}</span>
                </button>
                </MenuItem>
            </MenuContent>
        </MenuPositioner>
    </MenuRoot>
</template>
