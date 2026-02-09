import { ref, onUnmounted } from 'vue';

export function useCountdown(durationMs: number, callback?: () => void) {
    const timeRemaining = ref(durationMs);
    const isActive = ref(false);
    let rafId: number | null = null;
    let endTime: number | null = null;

    const tick = () => {
        const now = Date.now();
        timeRemaining.value = Math.max(0, (endTime || 0) - now);

        if (timeRemaining.value > 0) {
            rafId = requestAnimationFrame(tick);
        } else {
            stop()
            setTimeout(() => {
                callback?.()
            }, 0)
        }
    };

    const start = () => {
        if (isActive.value) return;
        isActive.value = true;
        endTime = Date.now() + timeRemaining.value;
        rafId = requestAnimationFrame(tick);
    };

    const stop = () => {
        if (rafId) {
            cancelAnimationFrame(rafId);
            rafId = null;
        }
        isActive.value = false;
    };

    const reset = (ms = durationMs) => {
        if (isActive.value) {
            // Re-up: add time to the current end time
            endTime = Date.now() + ms;
            timeRemaining.value = ms;
        } else {
            // Just reset the value when stopped
            timeRemaining.value = ms;
        }
    };

    onUnmounted(() => {
        stop();
    });

    return {
        timeRemaining,
        isActive,
        start,
        stop,
        reset
    };
}