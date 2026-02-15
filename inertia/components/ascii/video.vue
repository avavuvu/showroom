<script setup lang="ts">
import { nextTick, onMounted, onUnmounted } from 'vue';
import { useVideoToAscii } from '~/components/ascii/videoToAscii';
import flower from "~/icons/image/flower.webm"

const ascii = useVideoToAscii({
    fontSize: 12,
    colored: false,
    maxWidth: 1920,
    "charset": "emoji"


});

onMounted(async () => {

    const video = ascii.videoRef.value;
    if (video) {
        video.load();
        await new Promise((resolve) => {
            if (video.readyState >= 1) {
                resolve(null);
            } else {
                video.addEventListener('loadedmetadata', resolve, { once: true });
            }
        });

        video.play();
    }
});

onUnmounted(() => {
    ascii.canvasRef.value?.remove()
})
</script>

<template>
    <div :ref="ascii.containerRef" class="w-full h-full ">
        <video :ref="ascii.videoRef" :src="flower" loop playsInline crossOrigin="anonymous" style=" display:none" />
        <canvas :ref="ascii.canvasRef" class="w-full h-full object-cover" />
    </div>
</template>