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
</template>
