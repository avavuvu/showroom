<script setup lang="ts">
import Comment from '#models/comment'
import { computed } from 'vue';
import CommentList from './CommentList.vue'

const props = defineProps<{
    comment: Comment
    allComments?: Comment[]
}>()

const replies = computed(() =>
    props.allComments?.filter(c => c.parentId === props.comment.id) || []
)
</script>

<template>
    <div class="comment">
        <div class="comment-meta">
            <span class="author">{{ comment.user?.email ?? 'Anonymous' }}</span>
            <span class="date">{{ comment.createdAt.toRelative() }}</span>
        </div>
        <p class="comment-content">{{ comment.content }}</p>

        <!-- recurse -->
        <CommentList v-if="replies.length" :comments="replies" :all-comments="allComments" class="replies" />
    </div>
</template>