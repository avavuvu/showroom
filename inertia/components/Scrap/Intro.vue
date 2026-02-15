<script setup lang="ts">
import { CircleQuestionMarkIcon } from 'lucide-vue-next'
import { ref, onMounted } from 'vue'
import Button from '../blocks/Button.vue'
import Dialog from '../blocks/Dialog.vue'

const mounted = ref(false)
onMounted(() => {
    mounted.value = true
})
</script>

<template>
    <Dialog title="Scraps" as-child dismissText="I understand">
        <template #trigger>
            <Button as="button">
                <CircleQuestionMarkIcon />
            </Button>

        </template>
        <p>
            When you start a scrap, as soon as you stop writing for more than five seconds,
            <strong>
                everything you've written will be deleted.
            </strong>
        </p>
        <p>
            That might sound scary, but writer's block can be even scarier! Just remember,
            it's about getting words on the page, rather than getting words on
            the page perfectly. As such, the spellchecker, grammar checker, and all styling options have
            been
            disabled.
        </p>
    </Dialog>

    <!-- <Dialog.Root role="alertdialog">
        <Dialog.Trigger :class="styles.Root">
            <CircleQuestionMarkIcon />
        </Dialog.Trigger>
        <Teleport to="body" v-if="mounted">
            <Dialog.Backdrop :class="styles.Backdrop" />
            <Dialog.Positioner :class="styles.Positioner">
                <Dialog.Content :class="styles.Content" class="border shadow-2xl">
                    <Dialog.CloseTrigger :class="styles.CloseTrigger">
                        <XIcon />
                    </Dialog.CloseTrigger>
                    <Dialog.Title class="font-bold">
                        Scraps
                    </Dialog.Title>
                    <Dialog.Description :class="styles.Description">
                        <p>
                            As soon as you start writing, if you stop writing for more than five seconds,
                            <strong>
                                everything you've written will be deleted.
                            </strong>
                        </p>
                        <p>
                            It might sound scary, but writer's block can be even scarier! Just remember,
                            it's about getting words on the page, rather than getting words on
                            the page perfectly. As such, the spellchecker, grammar checker, and all styling options have
                            been
                            disabled.
                        </p>

                    </Dialog.Description>
                    <div :class="styles.Actions">
                        <Dialog.CloseTrigger class="py-2 underline">I understand</Dialog.CloseTrigger>
                    </div>
                </Dialog.Content>
            </Dialog.Positioner>
        </Teleport>
    </Dialog.Root> -->
</template>
