/// <reference path="../../adonisrc.ts" />
/// <reference path="../../config/inertia.ts" />

import { createSSRApp, h } from 'vue'
import type { DefineComponent } from 'vue'
import { createInertiaApp } from '@inertiajs/vue3'
import { resolvePageComponent } from '@adonisjs/inertia/helpers'

const appName = import.meta.env.VITE_APP_NAME || 'AdonisJS'

createInertiaApp({
  progress: { color: '#5468FF' },

  title: (title) => "Showroom",//`${title} - ${appName}`,

  resolve: async (name) => {
    if (name.startsWith("preview")) {
      await import('../css/email.css')
    } else {
      await import('../css/app.css')
    }

    return resolvePageComponent(
      `../pages/${name}.vue`,
      import.meta.glob<DefineComponent>('../pages/**/*.vue'),
    )
  },

  setup({ el, App, props, plugin }) {

    createSSRApp({ render: () => h(App, props) })

      .use(plugin)
      .mount(el)
  },
})