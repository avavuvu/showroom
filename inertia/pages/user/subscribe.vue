<script setup lang="ts">
import type User from '#models/user';
import { useForm, usePage } from '@inertiajs/vue3';

const page = usePage()

const { user } = defineProps<{ user: User }>()

const form = useForm({
    name: "",
    email: ""
})

const onsubmit = (event: Event) => {
    event.preventDefault()
    form.post("/subscribe")
}

</script>

<template>
    <ul v-if="page.props.errors">
        <li v-for="(error, name) in page.props.errors" :key="name">
            {{ name }}: {{ error }}
        </li>
    </ul>

    <div>
        Subscribe to {{ user.username }}.showroom.you
    </div>

    <form method="POST" @submit="onsubmit">
        <label>
            Email
            <input type="email" name="email" required v-model="form.email">
        </label>

        <label>
            Name
            <input type="text" name="name" v-model="form.name">
        </label>

        <button type="submit" :disabled="form.processing">Subscribe</button>
    </form>
</template>