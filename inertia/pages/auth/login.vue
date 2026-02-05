<script setup lang="ts">
import { useForm, usePage } from '@inertiajs/vue3'

const page = usePage()

const form = useForm({
  email: '',
  password: '',
  rememberMe: false,
})

const onsubmit = (event: Event) => {
  event.preventDefault()
  form.post('login')
}
</script>

<template>
  <h1>Login</h1>

  <ul v-if="page.props.errors">
    <li v-for="(error, name) in page.props.errors" :key="name">
      {{ name }}: {{ error }}
    </li>
  </ul>

  <form method="POST" @submit="onsubmit">
    <label>
      Email
      <input type="email" name="email" v-model="form.email">
    </label>

    <label>
      Password
      <input type="password" name="password" v-model="form.password">
    </label>

    <label>
      Remember Me?
      <input type="checkbox" name="remember-me" v-model="form.rememberMe">
    </label>

    <button type="submit" :disabled="form.processing">Login</button>
  </form>
</template>

<style scoped>
input {
  border: 1px solid black;
}

button {
  border: 1px solid black;
}
</style>
