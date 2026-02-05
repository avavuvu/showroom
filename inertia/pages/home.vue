<script setup lang="ts">
  import { useForm, usePage } from '@inertiajs/vue3'

  const page = usePage()

  const form = useForm()

  const onsubmit = (event: SubmitEvent) => {
    event.preventDefault()

    form.post("auth/logout")
  }

</script>

<template>
  {{JSON.stringify(page.props)}}

  <div v-if="page.props.auth">
      <form method="POST" {onsubmit}>
        <button type="submit">
          <h1> Hello {{page.props.auth.fullName}}</h1>
        </button>
      </form>

  </div>
  <div v-else>
    <h1> Hello World </h1>
  </div>

</template>

