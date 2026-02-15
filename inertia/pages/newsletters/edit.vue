<script setup lang="ts">
import type Newsletter from '#models/newsletter';
import { useForm, usePage } from '@inertiajs/vue3';
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import Toolbar from '~/components/editor/Toolbar.vue';
import { Placeholder } from '@tiptap/extensions';
import Blockquote from '@tiptap/extension-blockquote';
import Image from '@tiptap/extension-image'; import { useAutosave } from '~/directives/useAutosave';
import { onKeyStroke } from '@vueuse/core';
import { onMounted } from 'vue';
import { Circle } from 'lucide-vue-next';
import Button from '~/components/blocks/Button.vue';
import WritingLayout from '~/layouts/WritingLayout.vue';

const { newsletter } = defineProps<{
  newsletter: Newsletter
}>()

const form = useForm({
  title: newsletter.title || "",
  content: newsletter.content || "",
}).dontRemember("content", "title")

const { debouncedSave, save } = useAutosave(`/publish/${newsletter.id}`, form)

onKeyStroke('s', (event) => {
  if (event.ctrlKey || event.metaKey) {
    event.preventDefault()
    save()
  }
});

const editor = useEditor({
  content: form.content || "",
  extensions: [
    StarterKit.configure({
      blockquote: false,
    }),
    Blockquote.extend({
      addAttributes() {
        return {
          class: {
            default: null,
            parseHTML: element => element.getAttribute('class'),
            renderHTML: attributes => {
              if (!attributes.class) {
                return {}
              }
              return {
                class: attributes.class,
              }
            },
          },
        }
      },
    }),
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
      class: "outline-none",
    },
  },
  onUpdate({ transaction }) {
    if (!transaction.docChanged) return;

    form.content = editor.value!.getHTML()
    debouncedSave()
  },
})

onMounted(() => {
  form.title = newsletter.title || "";
  form.content = newsletter.content || "";

  editor.value?.commands.setContent(newsletter.content || "");
});

</script>

<template>
  <WritingLayout>
    <template #right-most>
      <div class="flex ">
        <div class="grid grid-cols-[auto_12px] px-3 py-1.5">
          <template v-if="form.isDirty">
            Saving
            <Circle class="w-full" />
          </template>
          <template v-else>
            Saved
          </template>
        </div>

        <Button size="sm" as="anchor" :href="`/publish/${newsletter.id}/send`">Publish</Button>
      </div>
    </template>
    <div class="relative min-h-screen font-[Times]">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 flex flex-col gap-4">


        <div v-if="editor" class="sticky top-12 z-10 pt-4 bg-white">
          <Toolbar :editor="editor" />
        </div>

        <div class="grid">
          <input type="text" name="title" class="text-4xl font-title font-bold mt-4 w-full border-black border-b"
            placeholder="Newsletter Title" v-model="form.title">
          <input type="text" name="subtitle"
            class="text-2xl font-title italic w-full border-none focus:ring-0 placeholder:text-gray-300"
            placeholder="Subtitle" v-model="form.title">

        </div>


        <article class="grid grid-cols-[1fr_70ch_1fr]">
          <editor-content :editor="editor" />

        </article>


      </div>
    </div>
  </WritingLayout>
</template>

<style>
.tiptap {
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

  p {
    margin-bottom: 1rem;
  }

  ul,
  ol {
    padding: 0 1rem;

  }

  h1,
  h2,
  h3 {
    font-family: var(--font-title);
  }



  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {
    text-wrap: pretty;
    font-weight: bold;

    strong {
      font-weight: normal;
    }
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
    margin: 2rem 0;
  }

  .pullquote {
    border: none;
    border-left: 4px solid var(--color-black);
    padding: 2rem;
    font-size: 1.5rem;
    font-style: italic;
    background: var(--color-surface-subtle);
    margin: 3rem 0;
  }
}
</style>
