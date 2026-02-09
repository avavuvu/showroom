<script setup lang="ts">
import type Scrap from '#models/scrap';
import { useForm, usePage } from '@inertiajs/vue3';
import { useEditor, EditorContent } from '@tiptap/vue-3';
import StarterKit from '@tiptap/starter-kit';
import { Placeholder } from '@tiptap/extensions';
import { useCountdown } from '~/directives/useCountdown';
import Intro from '~/components/Scrap/Intro.vue';
import binIcon from "~/icons/bin/trash.png"
import { onMounted, ref, Ref } from 'vue';
import { Link } from '@inertiajs/vue3';
import { animateScrap } from '~/components/Scrap/scrappedAnimation';

const page = usePage()

const { scrap } = defineProps<{
  scrap: Scrap
}>()

const form = useForm({
  content: scrap.content,
})

const editorParent: Ref<null | HTMLElement> = ref(null)
const targetElementRef: Ref<null | HTMLElement> = ref(null)

const state: Ref<"idle" | "writing" | "success" | "failed"> = ref(
  scrap.isSuccessful ? "success" : "idle"
)

const scrapSettings = {
  msTilDeath: 1000,
  msTilFinish: 5000,//5 * 60 * 1000, // five minutes
  bailButton: false,
}

const deathTimer = useCountdown(scrapSettings.msTilDeath, async () => {
  state.value = "failed"
  finishTimer.stop()

  const html = editor.value!.getHTML()
  editorParent.value!.innerHTML = html;
  editor.value?.commands.clearContent(false)
  editor.value?.setEditable(false)

  await animateScrap(editorParent.value!, targetElementRef.value!)
})

const finishTimer = useCountdown(scrapSettings.msTilFinish, async () => {
  state.value = "success"
  deathTimer.stop()
  await save()
})

const save = async () => {
  const content = editor!.value?.getText()

  form.content = content || ""

  form.put(`/scrap/${scrap.id}`)
}

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const editor = useEditor({
  content: form.content || "",
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: "Start writing quickly"
    }),
  ],
  editorProps: {
    attributes: {
      class: "focus:outline-none",
      spellcheck: "false"
    },
  },
  onUpdate({ transaction }) {
    if (scrap.isSuccessful) return;

    if (!transaction.docChanged) return;

    finishTimer.start()

    deathTimer.start()

    deathTimer.reset()
  }
})

onMounted(() => {
  finishTimer.reset(scrapSettings.msTilFinish);
  deathTimer.reset(scrapSettings.msTilDeath);
  state.value = "idle"
  editor.value?.setEditable(true)
});

</script>

<template>
  <div :style="{ opacity: 1 - (deathTimer.timeRemaining.value / scrapSettings.msTilDeath) }"
    class="evil-shadow fixed z-100 pointer-events-none w-screen h-screen">
  </div>

  <header class="sticky top-0 w-full bg-white border-b z-100 flex justify-between ">
    <Intro />
    <h2>{{ deathTimer.timeRemaining }}</h2>
    <h2>{{ finishTimer.timeRemaining }}</h2>

    <div class="grid grid-cols-2 w-md">
      <button class="aspect-square w-12">
        Export Markdown
      </button>
      <button class="aspect-square w-12">
        Copy text
      </button>
    </div>
  </header>

  <div class="relative min-h-screen font-[Times]">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex flex-col gap-4">
      <ul v-if="page.props.errors" class="text-red-500">
        <li v-for="(error, name) in page.props.errors" :key="name">
          {{ name }}: {{ error }}
        </li>
      </ul>

      <article class="grid grid-cols-[1fr_70ch_1fr]">
        <div>
          <p class="encouraging-words opacity-0 text-gray-400">Better luck next time!
            <Link href="/scrap" method="post" as="button" :preserve-state="false" class="underline cursor-pointer ">
              Click here to try again.
            </Link>
          </p>
          <editor-content :editor="editor" />
          <div ref="editorParent" class="editor-parent z-10">
          </div>

        </div>
      </article>
    </div>
  </div>

  <div ref="targetElementRef" class="fixed bottom-0 left-1/2 -translate-x-1/2 -z-10">
    <img class="bin-icon translate-y-full" :src="binIcon">
  </div>
</template>

<style>
.evil-shadow {
  background: radial-gradient(transparent 70%, rgb(206, 66, 66))
}

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
</style>
