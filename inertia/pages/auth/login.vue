<script setup lang="ts">
import { useForm, usePage } from '@inertiajs/vue3'
import Input from '~/components/blocks/Input.vue'
import Form from '~/components/blocks/Form.vue'
import Card from '~/components/blocks/Card.vue'

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
  <div class="flex flex-col items-center">
    <Card title="Sign in to Showroom">
      <Form :error="page.props.errors?.['E_INVALID_CREDENTIALS']" :onSubmit="onSubmit">
        <Input :error="page.props.errors?.['emailOrUsername']" type="text" placeholder="Your email or username"
          name="emailOrUsername" v-model="form.emailOrUsername" />

        <Input :error="page.props.errors?.['password']" type="password" placeholder="Your password" name="password"
          v-model="form.password" />

        <button type="submit" :disabled="form.processing">Login</button>
      </form>
    </Card>
  </div>
</template>

<style scoped>
input {
  border: 1px solid black;
}

button {
  border: 1px solid black;
}
</style>
