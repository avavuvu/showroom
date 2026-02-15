<script setup lang="ts">
import { useForm, usePage } from '@inertiajs/vue3'
import Input from '~/components/blocks/Input.vue'
import Form from '~/components/blocks/Form.vue'
import { Link } from '@inertiajs/vue3'
import Button from '~/components/blocks/Button.vue'

const page = usePage()

const form = useForm({
    emailOrUsername: '',
    password: '',
})

const onSubmit = (event: Event) => {
    event.preventDefault()
    form.post('login')
}


</script>

<template>
    <div class=" ">
        <div class="mb-4">
            <p class="text-subdued">Don't have an account? <a href="register" class="text-primary underline">Create
                    one here</a>
            </p>

        </div>

        <Form :error="page.props.errors?.['E_INVALID_CREDENTIALS']" :onSubmit="onSubmit">
            <Input :error="page.props.errors?.['emailOrUsername']" type="text" placeholder="Your email or username"
                name="emailOrUsername" v-model="form.emailOrUsername" />

            <Input :error="page.props.errors?.['password']" type="password" placeholder="Your password" name="password"
                v-model="form.password" />

            <Button :onClick="onSubmit" type="submit" :disabled="form.processing">Sign in</Button>
        </Form>
    </div>
</template>
