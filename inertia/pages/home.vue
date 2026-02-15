<script setup lang="ts">
import { useForm, usePage, Link } from '@inertiajs/vue3'
import Login from "~/components/auth/Login.vue"
import Dialog from '~/components/blocks/Dialog.vue'
import type User from '#models/user';
import Button from '~/components/blocks/Button.vue';
import logoWordmark from "~/icons/logo/bw.svg"
import logoIcon from "~/icons/logo/img.png"
// import exampleImage from "~/icons/image/dithering-effect.webp"
import Video from "~/components/ascii/video.vue"

const page = usePage()

import {
  useScroll,
  useWindowScroll
} from '@vueuse/core'
import { computed } from 'vue';
import Card from '~/components/blocks/Card.vue';

const { y } = useWindowScroll()

const small = computed(() => {
  if (y.value > 60) {
    return true
  }

  if (y.value === 0) {
    return false
  }

  return true
})

</script>

<template>
  <header class="sticky top-0 z-10 w-full flex justify-between bg-white border-b group " :class="small ? 'small' : ''">
    <div class="px-2 flex items-center">
      <img class="group-[.small]:h-8 h-24 transition-all duration-300" :src="logoIcon">

      <a class="p-4 transition-all">
        <img :src="logoWordmark" class="group-[.small]:h-8 h-24 transition-all duration-300 " alt="Showroom">
      </a>
    </div>
    <div class="flex justify-end items-end">


      <template v-if="!page.props.auth">
        <Button as="anchor" href="/register">Get Started</Button>

        <Dialog as-child title="Sign in to Showroom" alert>
          <template #trigger>
            <Button color="secondary">Login</Button>
          </template>
          <Login />
        </Dialog>
      </template>
      <template v-else>
        <Button as="anchor" variant="outline" href="/logout">Logout</Button>
        <Button as="anchor" href="/dashboard" color="secondary">Dashboard</Button>
      </template>

    </div>

  </header>


  <section class="grid grid-cols-1 relative w-screen h-screen ">

    <Video class="absolute -z-10" />

    <!-- <video class="col-start-1 col-end-2 row-start-1 row-end-2 -scale-x-100" :src="flower" autoplay loop muted></video> -->

    <div class="absolute grid md:grid-cols-2 lg:grid-cols-3 grid-rows-3 w-full h-full">
      <div class="p-4 col-start-3 row-start-2">
        <Card class="bg-white  ">
          <span class="">
            <strong>Showroom</strong> is like if <em>Instagram</em> wasn't evil
          </span>

          <ul class="ml-10">
            <li>Fix your creative block</li>
            <li>Send emails to your friends that they'll actually want to see</li>
            <li>Show off your work in your own personal <em>showroom</em></li>
          </ul>

        </Card>
      </div>


    </div>
  </section>

  <section>
    <h2>Why is <strong>Showroom</strong> not free?</h2>
    <p>Showroom is not a social media—you are not the product.</p>
    <p>Showroom is not trying to be Twitter. By paying money, you are making a commitment to yourself to write; and to
      share that writing with your friends. </p>

    <p>That doesn't mean Showroom is asocial. Join discussions on newsletters and sign their profiles.</p>
  </section>

  <section>
    <h2>Hardcore Mode</h2>
    <p>At the start of each month, a newsletter template will be created for you and – even if you haven't written
      anything – that newsletter will get sent.</p>
    <p>The threat of public humiliation is a powerful force</p>

  </section>

  <section>
    <h2>Scraps</h2>
    <p>Whenever you start writing a scrap, if you stop writing for more than five seconds, all of your work will be
      deleted. </p>
    <p>It might sound crazy, but it's </p>
  </section>




  <!-- <Link href="register">Register</Link> -->

</template>
