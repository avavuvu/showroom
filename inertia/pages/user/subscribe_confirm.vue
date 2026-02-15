<script setup lang="ts">
defineProps<{ success: boolean, name: string | null, username: string | null }>()

</script>

<template>
    <div v-if="success">
        Thank you for subscribing to {{ username }}
    </div>
    <div v-else>
        Failed to subscribe. If you believe this to be a mistake, please contact avadinhvu@gmail.com
    </div>
</template>