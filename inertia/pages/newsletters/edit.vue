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
import { onMounted, onUnmounted, watch } from 'vue';
import { Circle } from 'lucide-vue-next';
import Button from '~/components/blocks/Button.vue';
import WritingLayout from '~/layouts/WritingLayout.vue';
import ImageBlock from "~/components/editor/image-upload"
import Link from '@tiptap/extension-link';
import Collapsible from '~/components/blocks/Collapsible.vue';
import Alert from '~/components/blocks/Alert.vue';
import { SharedProps } from '@adonisjs/inertia/types';

const { newsletter } = defineProps<{
  newsletter: Newsletter
}>()

const { user } = usePage<SharedProps>().props

const form = useForm({
  title: newsletter.title || "",
  content: newsletter.content || "",
  subtitle: newsletter.subtitle || "",
  slug: newsletter.slug || (newsletter.title || "").toLowerCase().replace(/[^a-z\s]/g, '').replace(/\s+/g, '-')
}).dontRemember("content", "title", "slug")

const { debouncedSave, save } = useAutosave(`/publish/${newsletter.id}`, form)

watch(() => form.title, (newTitle) => {
  form.slug = newTitle.toLowerCase().replace(/[^a-z\s]/g, '').replace(/\s+/g, '-')
})

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
      link: false
    }),
    ImageBlock.configure({
      async onUpload(file: File) {
        const src = URL.createObjectURL(file);
        return {
          src,
        }
      },
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
    Link.extend({
      inclusive: false,
    }).configure({
      openOnClick: false,

    })
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

onUnmounted(() => {
  editor.value?.destroy()
})

</script>

<template>
  <WritingLayout>
    <template #header>
      <Alert v-if="newsletter.sent">
        <template #trigger>
          <span class=" italic">
            Editing published newsletter
          </span>
        </template>
        You are editing a newsletter that has been sent. It will update on your page, but not in subscriber's inboxes.
      </Alert>
    </template>
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

        <template v-if="newsletter.sent">
          <Button size="sm" as="anchor"
            :href="`http://${user?.username}.localtest.me:3333/posts/${newsletter.slug}`">View</Button>
        </template>
        <template v-else>
          <Button size="sm" as="anchor" :href="`/publish/${newsletter.id}/preview`">Publish</Button>
        </template>

      </div>
    </template>
    <div class="relative min-h-screen font-[Times]">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 flex flex-col gap-4">


        <div v-if="editor" class="sticky top-12 z-10 pt-4 bg-white">
          <Toolbar :editor="editor" />
        </div>

        <div class="grid">
          <Collapsible>
            <template #trigger-adjacent>
              <input type="text" name="title" class="text-4xl font-title font-bold w-full border-black border-b"
                placeholder="Newsletter Title" v-model="form.title">
            </template>
            <template #trigger>
              Slug
            </template>
            <span class="inline-flex w-full p-2 my-1 border border-dashed">
              <span class="text-subdued whitespace-nowrap">
                https://{{ newsletter?.user?.username || "username" }}.showroom.you/
              </span>
              <input placeholder="URL" v-model="form.slug" type="text" class="  w-full">
            </span>
          </Collapsible>

          <input type="text" name="subtitle"
            class="text-2xl font-title italic w-full border-none focus:ring-0 placeholder:text-gray-300"
            placeholder="Subtitle" v-model="form.subtitle">

        </div>

      </div>
      <editor-content :editor="editor" />

    </div>
  </WritingLayout>
</template>

<style>
.tiptap {
  display: grid;
  grid-template-columns: 1fr 70ch 1fr;
}

.tiptap {
  /* height: max(100%, 1200px); */
}

.tiptap>* {
  grid-column: 2;
}

.tiptap>[data-width="full"] {
  grid-column: 1;
  grid-column-start: 1;
  grid-column-end: 4;
}

.is-editor-empty:first-child::before {
  color: var(--color-gray-400);
  content: attr(data-placeholder);
  float: left;
  height: 0;
  pointer-events: none;
}

.tiptap {
  p {
    margin-bottom: 1rem;
  }

  a {
    text-decoration: underline;
    color: var(--color-primary);
  }

  ul,
  ol {
    padding: 0 1rem;
  }

  ul {
    list-style-type: disc;
  }

  ol {
    list-style-type: decimal;
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
    font-size: 1.5rem;
    font-style: italic;
    background: var(--color-surface-subtle);
  }
}
</style>
