<script setup lang="ts">
import { Dialog } from '@ark-ui/vue/dialog'
import { XIcon, CircleQuestionMarkIcon } from 'lucide-vue-next'
import { ref, onMounted } from 'vue'

const { title, asChild = false, dismissText, alert } = defineProps<{ title: string, asChild?: boolean, dismissText?: string, alert?: boolean }>()

const mounted = ref(false)
onMounted(() => {
    mounted.value = true
})
</script>

<template>
    <Dialog.Root :role="alert ? 'alertdialog' : 'dialog'">
        <Dialog.Trigger :as-child>
            <slot name="trigger" />
        </Dialog.Trigger>
        <Teleport to="body" v-if="mounted">
            <Dialog.Backdrop class="backdrop" />
            <Dialog.Positioner class="positioner">
                <Dialog.Content class="content border shadow-2xl flex flex-col p-4 bg-white">
                    <span class="inline-flex justify-between mb-4 w-full">
                        <Dialog.Title class="font-bold font-title ">
                            {{ title }}
                        </Dialog.Title>
                        <Dialog.CloseTrigger class="cursor-pointer ">
                            <XIcon />
                        </Dialog.CloseTrigger>
                    </span>
                    <slot />
                    <Dialog.CloseTrigger v-if="dismissText" class="py-2 underline">{{ dismissText }}
                    </Dialog.CloseTrigger>
                </Dialog.Content>
            </Dialog.Positioner>
        </Teleport>
    </Dialog.Root>
</template>

<style scoped>
.backdrop {
    position: fixed;
    inset: 0;
    z-index: calc(var(--demo-popover-z-index) + var(--layer-index, 0));

    &[data-state='open'] {
        animation: fade-in 0.15s ease-out;
    }

    &[data-state='closed'] {
        animation: fade-out 0.1s ease-in;
    }
}

.content {
    --px: 1.5rem;

    display: flex;
    flex-direction: column;
    align-items: flex-start;
    position: relative;
    width: 24rem;
    max-width: calc(100vw - 2rem);
    max-height: calc(100vh - 2rem);
    padding: var(--px);
    z-index: calc(var(--demo-popover-z-index) + var(--layer-index, 0));
    transition: transform 0.1s ease-in-out;
    /* Offset for scrollbar width when body scroll is locked (prevents visual shift) */
    translate: calc(-1 * var(--scrollbar-width, 0px) / 2) 0;

    &[data-has-nested] {
        transform: scale(calc(1 - var(--nested-layer-count) * 0.05));
    }

    &[data-state='open'] {
        animation: scale-fade-in 0.15s ease-out;
    }

    &[data-state='closed'] {
        animation: scale-fade-out 0.1s ease-in;
    }
}

.positioner {
    display: flex;
    align-items: center;
    justify-content: center;
    position: fixed;
    inset: 0;
    z-index: calc(var(--demo-popover-z-index) + var(--layer-index, 0));
    /* Prevent overscroll bounce on iOS */
    overscroll-behavior-y: none;
    /* Reserve space for scrollbar to prevent layout shift */
    scrollbar-gutter: stable both-edges;
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

@keyframes scale-fade-in {
    from {
        opacity: 0;
        transform: scale(0.95);
    }

    to {
        opacity: 1;
        transform: scale(1);
    }
}

@keyframes scale-fade-out {
    from {
        opacity: 1;
        transform: scale(1);
    }

    to {
        opacity: 0;
        transform: scale(0.95);
    }
}
</style>
