<script setup lang="ts">
import Button from '../blocks/Button.vue';
import Dialog from '../blocks/Dialog.vue';
import Form from '../blocks/Form.vue';
import Input from '../blocks/Input.vue';

defineProps<{ username: string }>()

const onSubmit = () => {

}

</script>

<template>
    <Dialog alert as-child fullscreen>
        <template #trigger>
            <Button>
                Subscribe
            </Button>
        </template>

        <h2 class="font-tile text-2xl font-bold">Subscribe to {{ username }}</h2>

        <p>By subscribing, you agree to recieve emails from {{ username }}.</p>

        <Form :onSubmit>
            <Input type="text" name="name" placeholder="Your name (Optional)" />
            <Input type="email" name="email" placeholder="Your email" />
            <Button type="submit" as="button">Susbcribe</Button>
        </Form>

    </Dialog>
</template>