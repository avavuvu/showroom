<script setup lang="ts">
import type Newsletter from '#models/newsletter';
import Button from '~/components/blocks/Button.vue';
import Card from '~/components/blocks/Card.vue';
import Dialog from '~/components/blocks/Dialog.vue';
import AuthenticatedLayout from '~/layouts/AuthenticatedLayout.vue';

const { newsletter, html, username, fullname } = defineProps<{
    newsletter: Newsletter,
    username: string,
    fullname: string,
    html: string
}>()


</script>

<template>
    <AuthenticatedLayout>
        <div class="py-8 px-4 sm:px-6 lg:px-8">
            <Card class="max-w-4xl mx-auto">
                <Card class="shadow grid gap-2 grid-rows-3">
                    <div class="grid grid-cols-[4rem_auto]">
                        <span class="text-subdued">SUBJ:</span><span> {{ newsletter.title }}</span>
                    </div>
                    <div class="grid grid-cols-[4rem_auto]">
                        <span class="text-subdued">FROM:</span><span> {{ fullname }} <span class="text-subdued">
                                ⟪{{ username }}@showroom.you⟫
                            </span></span>
                    </div>
                    <div class="grid grid-cols-[4rem_auto]">
                        <span class="text-subdued">TO: </span><span>Erica Example <span class="text-subdued">
                                ⟪erica@example.com⟫</span></span>
                    </div>
                    <Dialog :title="`Send ${newsletter.title}`" as-child>
                        <template #trigger>
                            <Button>
                                Send
                            </Button>
                        </template>
                        <Button>
                            Send to everyone now
                        </Button>
                    </Dialog>
                </Card>

                <div class="email-content" v-html="html"></div>
            </Card>
        </div>
    </AuthenticatedLayout>
</template>

<style scoped>
.email-content * {
    all: revert;
}
</style>