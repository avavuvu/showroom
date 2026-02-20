<script setup lang="ts">
import { Dialog } from '@ark-ui/vue/dialog'
import { XIcon, CircleQuestionMarkIcon } from 'lucide-vue-next'
import { ref, onMounted } from 'vue'

const {
    title,
    asChild = false,
    dismissText,
    alert,
    fullscreen = false,
    lazy = false,
} = defineProps<{
    title?: string,
    asChild?: boolean,
    dismissText?: string,
    alert?: boolean,
    fullscreen?: boolean,
    lazy?: boolean,
}>()

const mounted = ref(false)
onMounted(() => {
    mounted.value = true
})
</script>

<template>
    <Dialog.Root :role="alert ? 'alertdialog' : 'dialog'" :lazy-mount="lazy">
        <Dialog.Trigger :as-child>
            <slot name="trigger" />
        </Dialog.Trigger>
        <Teleport to="body" v-if="mounted">
            <Dialog.Backdrop class="backdrop" />
            <Dialog.Positioner class="positioner" :class="fullscreen ? 'full-screen' : 'center-screen'">
                <Dialog.Content class="content flex flex-col p-4 bg-white"
                    :class="[fullscreen ? 'content-fullscreen' : 'border shadow-2xl']">
                    <span class="dialog-header inline-flex items-center mb-4 w-full">
                        <Dialog.Title v-if="title" class="font-bold font-title ">
                            {{ title }}
                        </Dialog.Title>
                        <Dialog.CloseTrigger class="cursor-pointer ml-auto">
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

.content-fullscreen {
    width: 100vw;
    width: 100dvw;
    height: 100vh;
    height: 100dvh;
    max-width: none;
    max-height: none;
    border-radius: 0;
    translate: 0 0;
    justify-content: center;
    align-items: center;
}

.content-fullscreen .dialog-header {
    position: absolute;
    top: 0;
    left: 0;
    padding: var(--px);
}

.full-screen {
    position: fixed;
    width: 100vw;
    width: 100dvw;
    height: 100vh;
    height: 100dvh;
}

.center-screen {
    display: flex;
    align-items: center;
    justify-content: center;
    position: fixed;
}

.positioner {
    z-index: 100;

    inset: 0;
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
