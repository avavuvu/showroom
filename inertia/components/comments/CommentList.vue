<script setup lang="ts">
import Comment from '#models/comment'
import CommentItem from './CommentItem.vue'

defineProps<{
    comments?: Comment[]
    allComments?: Comment[]
}>()
</script>

<template>
    <div class="comment-list">
        <CommentItem v-for="comment in comments" :key="comment.id" :comment="comment" :all-comments="allComments" />
    </div>
</template>