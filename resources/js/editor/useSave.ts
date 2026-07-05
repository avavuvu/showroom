import { useDebounceFn } from "@vueuse/core";
import { ref } from "vue";

const dispatch = (detail: string) =>
    window.dispatchEvent(new CustomEvent("save-status", { detail }));

export const useSave = (putFunction: () => Promise<boolean>, ms = 2_000) => {
    const isDirty = ref(false);

    const markDirty = () => {
        isDirty.value = true;
        dispatch("Saving…");
    };

    const save = async () => {
        dispatch("Saving…");
        const ok = await putFunction();
        isDirty.value = false;
        dispatch(ok ? "Saved" : "Error saving");
    };

    const debouncedSave = useDebounceFn(save, ms);

    return { save, debouncedSave, isDirty, markDirty };
};
