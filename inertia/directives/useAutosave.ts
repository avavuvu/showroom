import { useDebounceFn } from "@vueuse/core"
import { InertiaForm } from "@inertiajs/vue3"
import { onBeforeUnmount } from "vue";

export const useAutosave = (url: string, form: InertiaForm<any>, ms = 2_000) => {
    const save = () => {
        if (form.processing) { return }

        form.put(url, {
            preserveScroll: true,
            preserveState: true,
        });
    };

    const debouncedSave = useDebounceFn(save, ms);

    onBeforeUnmount(() => {
        save()
    })

    return {
        save,
        debouncedSave,
        isSaving: form.processing
    };
}