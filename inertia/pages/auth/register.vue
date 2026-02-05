<script setup lang="ts">
import { useForm, usePage } from '@inertiajs/vue3'
import { ref } from 'vue'

const page = usePage()

const form = useForm({
  fullName: '',
  email: '',
  password: '',
  username: ''
})

const hasEditedUsername = ref(false)
const username = ref('')

const onEditedUsername = (event: Event) => {
  hasEditedUsername.value = true
  const target = event.target as HTMLInputElement
  username.value = target.value
}

const onEditedFullname = (event: Event) => {
  if (hasEditedUsername.value) {
    return
  }
  const target = event.target as HTMLInputElement
  username.value = target.value
    .normalize('NFD')
    .replace(/[^a-zA-Z0-9\s-]/g, '')
    .replace(/\s+/g, '')
    .toLowerCase()
}

const onsubmit = (event: Event) => {
  event.preventDefault()
  form.username = username.value
  form.post('register')
}
</script>

<template>
  <h1>Register</h1>

  <ul v-if="page.props.errors">
    <li v-for="(error, name) in page.props.errors" :key="name">
      {{ name }}: {{ error }}
    </li>
  </ul>

  <form method="POST" @submit="onsubmit">
    <label>
      Full name
      <input
        type="text"
        name="fullName"
        v-model="form.fullName"
        @input="onEditedFullname"
      >
    </label>

    <label>
      Your URL
      <span class="email-preview">
        <span>news@</span>
        <input
          type="text"
          name="username"
          v-model="username"
          @input="onEditedUsername"
        >
        <span>.showroom.name</span>
      </span>
    </label>

    <label>
      Email
      <input type="email" name="email" v-model="form.email">
    </label>

    <label>
      Password
      <input type="password" name="password" v-model="form.password">
    </label>

    <button type="submit" :disabled="form.processing">Register</button>
  </form>
</template>

<style scoped>
input {
  border: 1px solid black;
}

form {
  display: grid;
  width: 600px;
}

.email-preview {
  display: flex;
}

.email-preview span {
  color: gray;
}

label {
  display: grid;
  grid-template-columns: 1fr 1fr;
}

button {
  border: 1px solid black;
}
</style>
