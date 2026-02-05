<script setup lang="ts">
import { Milkdown, useEditor } from "@milkdown/vue"
import { Crepe } from "@milkdown/crepe";
import { useForm } from "@inertiajs/vue3";

import { imageBlockConfig } from "@milkdown/kit/component/image-block"

import "@milkdown/crepe/theme/common/style.css";
import { listenerCtx } from "@milkdown/kit/plugin/listener";
import { ref } from "vue";
import type Newsletter from "#models/newsletter";

const props = defineProps<{ newsletter: Newsletter }>()

const markdown = ref(props.newsletter.content || "")
const form = useForm({
  title: props.newsletter.title || "",
  content: props.newsletter.content || ""
})

const save = async () => {
  form.put(`/publish/${props.newsletter.id}`)
}

const { get } = useEditor((root) => {
  const crepeBuilder = new Crepe({
    root,
  })

  crepeBuilder.editor.config(context => {
    context.get(listenerCtx).markdownUpdated((_, md) => {
      markdown.value = md
      form.content = md
    })

    context.update(imageBlockConfig.key, (defaultConfig) => ({
      ...defaultConfig,
      onUpload: async (file) => {
        const formData = new FormData()
        formData.append('image', file)

        const uploadResponse = await fetch('/images', {
          method: 'POST',
          body: formData,
          headers: {
            'X-XSRF-TOKEN': document.cookie.match(/XSRF-TOKEN=([^;]+)/)?.[1] || ''
          }
        })

        const { url } = await uploadResponse.json()
        return url
      },
    }))
  })



  return crepeBuilder
})


</script>

<template>
  <input type="text" name="title" class="text-4xl" v-model="form.title">
  <button :disabled="form.processing" @click="save">Save</button>
  <article>

    <Milkdown />

    <code>
      {{ markdown }}
    </code>

  </article>
</template>

<style lang="css">
.milkdown {
  border: 2px solid black;
  height: max(100%, 1200px);
}

.milkdown {
  --crepe-color-background: #ffffff;
  --crepe-color-on-background: #000000;
  --crepe-color-surface: #f7f7f7;
  --crepe-color-surface-low: #ededed;
  --crepe-color-on-surface: #1c1c1c;
  --crepe-color-on-surface-variant: #4d4d4d;
  --crepe-color-outline: #a8a8a8;
  --crepe-color-primary: #333333;
  --crepe-color-secondary: #cfcfcf;
  --crepe-color-on-secondary: #000000;
  --crepe-color-inverse: #f0f0f0;
  --crepe-color-on-inverse: #1a1a1a;
  --crepe-color-inline-code: #ba1a1a;
  --crepe-color-error: #ba1a1a;
  --crepe-color-hover: #e0e0e0;
  --crepe-color-selected: #d5d5d5;
  --crepe-color-inline-area: #cacaca;

  --crepe-font-title: 'Noto Serif', Cambria, 'Times New Roman', Times, serif;
  --crepe-font-default: 'Noto Sans', Arial, Helvetica, sans-serif;
  --crepe-font-code:
    'Space Mono', Fira Code, Menlo, Monaco, 'Courier New', Courier, monospace;

  --crepe-shadow-1:
    0px 1px 3px 1px rgba(0, 0, 0, 0.15), 0px 1px 2px 0px rgba(0, 0, 0, 0.3);
  --crepe-shadow-2:
    0px 2px 6px 2px rgba(0, 0, 0, 0.15), 0px 1px 2px 0px rgba(0, 0, 0, 0.3);
}

.milkdown .editor {
  height: 100%;
  padding: 4rem 8rem;
}

article {
  display: grid;
  grid-template-columns: 1fr 1fr;
}

article>* {
  /* grid-column-start: 2; */
}
</style>
