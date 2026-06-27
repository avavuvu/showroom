import { useDebounceFn } from "@vueuse/core";
import { ref } from "vue";

export const useSave = (putFunction: () => Promise<void>, ms = 2_000) => {
    const isSaving = ref(false);
    const isDirty = ref(false);

    const save = async () => {
        isDirty.value = false;
        isSaving.value = true;
        await putFunction();
        isSaving.value = false;
    };

    const debouncedSave = useDebounceFn(save, ms);

    return { save, debouncedSave, isSaving, isDirty };
};
