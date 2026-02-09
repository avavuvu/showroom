<script setup lang="ts">
import type Newsletter from '#models/newsletter';
import { useForm, usePage } from '@inertiajs/vue3';
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import Toolbar from '~/components/editor/Toolbar.vue';
import { Placeholder } from '@tiptap/extensions';
import Image from '@tiptap/extension-image'; ``

const page = usePage()

const { newsletter } = defineProps<{
  newsletter: Newsletter
}>()

const form = useForm({
  title: newsletter.title,
  content: newsletter.content,
})


const save = async () => {
  const content = editor!.value?.getHTML()

  form.content = content || ""

  form.put(`/publish/${newsletter.id}`)
}

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const editor = useEditor({
  content: form.content || "",
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: "Write something..."
    }),
    Image.configure({
      resize: {
        enabled: false,
        alwaysPreserveAspectRatio: true,
      },
      inline: false
    }),
  ],
  editorProps: {
    attributes: {
      class: "",
    },
  },
  onUpdate(props) {

  },
})

</script>

<template>
  <div class="relative min-h-screen font-[Times]">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex flex-col gap-4">
      <ul v-if="page.props.errors" class="text-red-500">
        <li v-for="(error, name) in page.props.errors" :key="name">
          {{ name }}: {{ error }}
        </li>
      </ul>

      <div v-if="editor">
        <Toolbar :editor="editor" />
      </div>

      <div class="grid">
        <input type="text" name="title" class="text-4xl mt-4 w-full border-black border-b"
          placeholder="Newsletter Title" v-model="form.title">
        <input type="text" name="subtitle" class="text-2xl w-full border-none focus:ring-0 placeholder:text-gray-300"
          placeholder="Subtitle" v-model="form.title">

      </div>
      <div class="flex justify-between items-center bg-white p-4 ">


        <button :disabled="form.processing" @click="save" class="ml-4 inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium
          text-white bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 transition-colors">
          {{ form.processing ? 'Saving...' : 'Save' }}
        </button>
      </div>


      <article class="grid grid-cols-[1fr_70ch_1fr]">
        <editor-content :editor="editor" />

      </article>

    </div>
  </div>
</template>

<style>
article {
  height: max(100%, 1200px);
}

article>* {
  grid-column: 2;

}

.is-editor-empty:first-child::before {
  color: var(--color-gray-400);
  content: attr(data-placeholder);
  float: left;
  height: 0;
  pointer-events: none;
}

ul,
ol {
  padding: 0 1rem;

  li p {}
}

/* Heading styles */
h1,
h2,
h3,
h4,
h5,
h6 {
  text-wrap: pretty;
}

h1 {
  font-size: 1.4rem;
}

h2 {
  font-size: 1.2rem;
}

h3 {
  font-size: 1.1rem;
}

h4,
h5,
h6 {
  font-size: 1rem;
}

blockquote {
  border-left: 3px solid var(--gray-3);
  margin: 1.5rem 0;
  padding-left: 1rem;
}

hr {
  border: none;
  border-top: 1px solid var(--gray-2);
  margin: 2rem 0;
}
</style>
