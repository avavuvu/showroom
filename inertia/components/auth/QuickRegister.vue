<script setup lang="ts">
import { useForm, usePage } from '@inertiajs/vue3'
import Input from '~/components/blocks/Input.vue'
import Form from '~/components/blocks/Form.vue'
import Button from '~/components/blocks/Button.vue'

const page = usePage()

const form = useForm({
    username: '',
    email: '',
})

const onSubmit = (event: Event) => {
    event.preventDefault()
    form.post('http://localtest.me:3333/quick')
}


</script>

<template>
    <Form :onSubmit="onSubmit">
        <Input :error="page.props.errors?.['username']" type="text" placeholder="Your handle" name="username"
            v-model="form.username" />

        <Input :error="page.props.errors?.['email']" type="text" placeholder="Your email" name="email"
            v-model="form.email" />

        <Button :onClick="onSubmit" type="submit" :disabled="form.processing">Post</Button>
    </Form>
</template>
