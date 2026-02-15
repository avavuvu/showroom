<script setup lang="ts">

const { name } = defineProps<{ name: string }>()
</script>

<template>
    Thanks for subscribing {{ name }}!
</template>