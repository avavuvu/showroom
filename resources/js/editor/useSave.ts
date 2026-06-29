import { useDebounceFn } from "@vueuse/core";
import { ref } from "vue";

export const useSave = (putFunction: () => Promise<boolean>, ms = 2_000) => {
    const isSaving = ref(false);
    const isDirty = ref(false);
    const networkError = ref(false);

    const save = async () => {
        isSaving.value = true;
        const ok = await putFunction();
        isSaving.value = false;

        if (ok) {
            isDirty.value = false;
            networkError.value = false;
        } else {
            networkError.value = true;
        }
    };

    const debouncedSave = useDebounceFn(save, ms);

    return { save, debouncedSave, isSaving, isDirty, networkError };
};
