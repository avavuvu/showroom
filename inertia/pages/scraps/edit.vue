<script setup lang="ts">
import type Scrap from '#models/scrap';
import { useForm, usePage } from '@inertiajs/vue3';
import { useEditor, EditorContent } from '@tiptap/vue-3';
import Document from '@tiptap/extension-document'
import Paragraph from '@tiptap/extension-paragraph'
import Text from '@tiptap/extension-text'
import { Placeholder } from '@tiptap/extensions';
import { useCountdown } from '~/directives/useCountdown';
import Intro from '~/components/Scrap/Intro.vue';
import binIcon from "~/icons/bin/trash.png"
import { onMounted, ref, Ref } from 'vue';
import { Link } from '@inertiajs/vue3';
import { animateScrap } from '~/components/Scrap/scrappedAnimation';
import { useAutosave } from '~/directives/useAutosave';
import WritingLayout from '~/layouts/WritingLayout.vue';
import Button from '~/components/blocks/Button.vue';
import { FileDown, Newspaper } from 'lucide-vue-next';
import ScrapIcon from '~/components/icons/Scrap.vue';

const page = usePage()

const { scrap } = defineProps<{
  scrap: Scrap
}>()

const form = useForm({
  content: scrap.content,
  isSuccessful: scrap.isSuccessful // false on new scrap, true on old scrap
}).dontRemember("content")

const { debouncedSave, save } = useAutosave(`/scrap/${scrap.id}`, form)

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

const finishTimer = useCountdown(scrapSettings.msTilFinish, () => {
  state.value = "success"
  form.isSuccessful = true
  deathTimer.stop()
  save()
})


const editor = useEditor({
  content: form.content || "",
  extensions: [
    Paragraph,
    Document,
    Text,
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
  onUpdate({ transaction, editor }) {
    form.content = editor.getHTML()


    if (!transaction.docChanged) return;

    if (state.value === "success") {
      debouncedSave()
    }

    if (scrap.isSuccessful) return;


    finishTimer.start()

    deathTimer.start()

    deathTimer.reset()
  }
})

onMounted(() => {
  finishTimer.reset(scrapSettings.msTilFinish);
  deathTimer.reset(scrapSettings.msTilDeath);
  state.value = scrap.isSuccessful ? "success" : "idle"
  editor.value?.setEditable(true)
  form.content = scrap.content || "";
  editor.value?.commands.setContent(scrap.content || "");
});



</script>

<template>
  <WritingLayout>
    <template #right-most>
      <div class="flex gap-2" v-if="state === 'success'">
        <Button size="sm" variant="outline" as="button">
          <FileDown></FileDown>
          Download as .md
        </Button>
        <Button size="sm" variant="outline" as="button">
          <Newspaper></Newspaper>
          Convert into newsletter
        </Button>
        <Button size="sm" variant="outline" as="button">
          <ScrapIcon size="24px" />
          Start a new Scrap
        </Button>
        <Intro />

      </div>
    </template>
    <div :style="{ opacity: 1 - (deathTimer.timeRemaining.value / scrapSettings.msTilDeath) }"
      class="evil-shadow fixed z-100 pointer-events-none w-screen h-screen">
    </div>

    <div class="relative min-h-screen font-[Times]">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex flex-col gap-4">
        <ul v-if="page.props.errors" class="text-red-500">
          <li v-for="(error, name) in page.props.errors" :key="name">
            {{ name }}: {{ error }}
          </li>
        </ul>

        <article class="grid grid-cols-[1fr_70ch_1fr]">
          <div>
            <p :class="{ 'failed': state === 'failed' }" class="encouraging-words opacity-0 text-gray-400">Better luck
              next
              time!
              <Link href="/scrap" method="post" as="button" :preserve-state="false" class="underline cursor-pointer ">
                Click here to try again.
              </Link>
            </p>
            <editor-content v-model="form.content" :editor="editor" />
            <div ref="editorParent" class="editor-parent z-10">
            </div>



          </div>
        </article>
      </div>
    </div>

    <div ref="targetElementRef" class="fixed bottom-0 left-1/2 -translate-x-1/2 -z-10">
      <img class="bin-icon translate-y-full" :src="binIcon">
    </div>
  </WritingLayout>
</template>

<style>
.encouraging-words {
  pointer-events: none;
  display: none;

}

.failed {
  pointer-events: all;
  display: inline;
}



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
