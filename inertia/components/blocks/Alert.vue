<script setup lang="ts">
import { Collapsible } from '@ark-ui/vue/collapsible'
import { ChevronRightIcon, ChevronUpIcon } from 'lucide-vue-next'
import { ref } from 'vue';

const open = ref(true)
</script>

<template>
    <Collapsible.Root class="group bg-primary text-surface-subtle text-center">
        <Collapsible.Trigger class="flex justify-center items-center w-full cursor-pointer">
            <Collapsible.Indicator class="inline-flex gap-2">
                <slot name="trigger" />
                <ChevronUpIcon class="group-data-[state=open]:rotate-180 transition-transform" />
            </Collapsible.Indicator>

        </Collapsible.Trigger>

        <Collapsible.Content class="">
            <slot></slot>
        </Collapsible.Content>
    </Collapsible.Root>
</template>

<style scoped>
@keyframes expand {
    from {
        height: var(--collapsed-height, 0);
    }

    to {
        height: var(--height);
    }
}

@keyframes collapse {
    from {
        height: var(--height);
    }

    to {
        height: var(--collapsed-height, 0);
    }
}


[data-scope='collapsible'][data-part='content'] {
    &[data-state='open'] {
        animation: expand 250ms,
            fade-in 120ms ease-out;
    }

    &[data-state='closed'] {
        animation: collapse 250ms,
            fade-out 120ms ease-out;
    }
}

@keyframes fade-in {
    from {
        opacity: 0;
    }

    to {
        opacity: 1;
    }
}

@keyframes fade-out {
    from {
        opacity: 1;
    }

    to {
        opacity: 0;
    }
}
</style>