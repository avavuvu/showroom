<script setup lang="ts">
import { useForm, usePage } from '@inertiajs/vue3';
import AuthenticatedLayout from '~/layouts/AuthenticatedLayout.vue';
import Input from '~/components/blocks/Input.vue';
import Button from '~/components/blocks/Button.vue';
import StandardCollapsible from '~/components/blocks/StandardCollapsible.vue';
import { computed, ref } from 'vue';
import { SharedProps } from '@adonisjs/inertia/types';
import { LucidePencil } from 'lucide-vue-next';

const page = usePage<SharedProps>();
const user = computed(() => page.props.user);

defineProps<{
    defaultProfileImages: string[]
}>()

const form = useForm({
    fullName: user.value?.fullName || '',
    username: user.value?.username || '',
    email: user.value?.email || '',
    profileImageUrl: user.value?.profileImageUrl || ''
});

const submit = () => {
    form.put('/profile', {
        preserveScroll: true,
    })
}
const isUploading = ref(false);
const fileInput = ref<HTMLInputElement | null>(null);

const handleImageUpload = async (event: Event) => {
    const input = event.target as HTMLInputElement
    if (!input.files || input.files.length === 0) return

    const file = input.files[0]
    const formData = new FormData()
    formData.append('avatar', file)

    isUploading.value = true

    try {
        const response = await fetch('/profile/image', {
            method: 'POST',
            body: formData,
            headers: {
                'Accept': 'application/json',
                'X-XSRF-TOKEN': decodeURIComponent(document.cookie.match(/XSRF-TOKEN=([^;]+)/)?.[1] || '')
            }
        })

        if (!response.ok) throw new Error('Upload failed')

        const data = await response.json()
        form.profileImageUrl = data.key
        form.isDirty = true
    } catch (error) {
        console.error('Error uploading image:', error)
    } finally {
        isUploading.value = false
        input.value = ''
    }
}
</script>

<template>
    <AuthenticatedLayout>
        <div class="max-w-2xl mx-auto py-12 px-6">
            <h1 class="text-3xl font-bold mb-8">Edit Profile</h1>

            <form @submit.prevent="submit" class="space-y-6">
                <div class="space-y-2">
                    <label class="text-sm font-medium">Profile Image</label>
                    <StandardCollapsible unmount-on-exit>
                        <template #trigger>
                            <button
                                class="relative group cursor-pointer w-20 h-20 p-0 border-none bg-transparent appearance-none text-left">
                                <div class="w-full h-full flex items-center justify-center overflow-hidden">
                                    <img v-if="form.profileImageUrl"
                                        :src="form.profileImageUrl.startsWith('http') ? form.profileImageUrl : `https://showroom-uploads.s3.amazonaws.com/${form.profileImageUrl}`"
                                        class="w-full h-full object-cover" alt="Profile" />
                                    <div v-else class="text-subdued text-xs text-center px-1">Upload</div>
                                </div>

                                <div v-if="isUploading"
                                    class="absolute inset-0 rounded-full flex items-center justify-center z-10">
                                    <span
                                        class="w-4 h-4 border-2 border-black border-t-transparent rounded-full animate-spin"></span>
                                </div>

                                <div class="absolute -bottom-1 -right-1 bg-white rounded-full p-1 border border-border">
                                    <LucidePencil :size="16" />
                                </div>
                            </button>
                        </template>

                        <div class="pt-4">
                            <div class="p-4 border border-border shadow space-y-4">
                                <div>
                                    <div class="grid grid-cols-5 gap-2">
                                        <button v-for="img in defaultProfileImages" :key="img" type="button"
                                            @click="form.profileImageUrl = img; form.isDirty = true"
                                            class="aspect-square overflow-hidden cursor-pointer" :class="form.profileImageUrl === img
                                                ? 'bg-surface-hover'
                                                : 'border-transparent hover:bg-surface-hover'">
                                            <img :src="`https://showroom-uploads.s3.amazonaws.com/${img}`"
                                                class="w-full h-full object-cover" />
                                        </button>
                                    </div>
                                </div>

                                <div class="border-t border-border pt-4">
                                    <Button type="button" variant="outline" size="sm" class="w-full justify-center"
                                        @click="fileInput?.click()" :disabled="isUploading">
                                        <input type="file" ref="fileInput" class="hidden" accept="image/*"
                                            @change="handleImageUpload" />
                                        {{ isUploading ? 'Uploading...' : 'Upload your own' }}
                                    </Button>
                                    <p class="text-[10px] text-subdued mt-2 text-center">Recommended size 400x400px
                                    </p>
                                </div>
                            </div>
                        </div>
                    </StandardCollapsible>
                </div>

                <div class="space-y-2">
                    <label class="text-sm font-medium">Full Name</label>
                    <Input type="text" name="fullName" placeholder="Your Name" v-model="form.fullName"
                        :error="form.errors.fullName" />
                </div>

                <div class="space-y-2">
                    <label class="text-sm font-medium">Username</label>
                    <Input type="text" name="username" placeholder="username" v-model="form.username"
                        :error="form.errors.username" />
                </div>

                <div class="space-y-2">
                    <label class="text-sm font-medium">Email</label>
                    <Input type="email" name="email" placeholder="you@example.com" v-model="form.email"
                        :error="form.errors.email" />
                </div>



                <StandardCollapsible :open="form.isDirty" :unmountOnExit="true">
                    <div class="pt-6 border-t mt-6 flex justify-end">
                        <Button type="submit" :disabled="form.processing">
                            Save Changes
                        </Button>
                    </div>
                </StandardCollapsible>
            </form>
        </div>
    </AuthenticatedLayout>
</template>
