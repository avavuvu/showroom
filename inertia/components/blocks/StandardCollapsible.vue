<script setup lang="ts">
import { Collapsible } from '@ark-ui/vue/collapsible'

const props = withDefaults(defineProps<{
    unmountOnExit?: boolean
    lazyMount?: boolean
    disabled?: boolean
}>(), {
    disabled: false
})

const emits = defineEmits<{
    (e: 'update:open', value: boolean): void
}>()
</script>

<template>
    <Collapsible.Root :unmountOnExit="props.unmountOnExit" :lazyMount="props.lazyMount" :disabled="props.disabled"
        @update:open="(val) => emits('update:open', val)">
        <Collapsible.Trigger v-if="$slots.trigger" as-child>
            <slot name="trigger" />
        </Collapsible.Trigger>

        <Collapsible.Content class="overflow-hidden">
            <slot />
        </Collapsible.Content>
    </Collapsible.Root>
</template>

<style scoped>
@keyframes slideDown {
    from {
        height: 0;
        opacity: 0;
    }

    to {
        height: var(--height);
        opacity: 1;
    }
}

@keyframes slideUp {
    from {
        height: var(--height);
        opacity: 1;
    }

    to {
        height: 0;
        opacity: 0;
    }
}

[data-scope='collapsible'][data-part='content'] {
    &[data-state='open'] {
        animation: slideDown 300ms cubic-bezier(0.87, 0, 0.13, 1);
    }

    &[data-state='closed'] {
        animation: slideUp 300ms cubic-bezier(0.87, 0, 0.13, 1);
    }
}
</style>
