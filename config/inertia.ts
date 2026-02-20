import { defineConfig } from '@adonisjs/inertia'
import type { InferSharedProps } from '@adonisjs/inertia/types'

const inertiaConfig = defineConfig({
  /**
   * Path to the Edge view that will be used as the root view for Inertia responses
   */
  rootView: 'inertia_layout',

  /**
   * Data that should be shared with all rendered pages
   */
  sharedData: {
    auth: (context) => context.auth.user
      ? {
        id: context.auth.user.id,
        fullName: context.auth.user.fullName,
        email: context.auth.user.email,
      }
      : null,
    user: (context) => context.inertia.always(() => context.auth.user),
    subdomainUser: (context) => context.inertia.always(() => context.subdomainUser),
  },

  /**
   * Options for the server-side rendering
   */
  ssr: {
    enabled: true,
    entrypoint: 'inertia/app/ssr.ts'
  }
})

export default inertiaConfig

import type User from '#models/user'

declare module '@adonisjs/inertia/types' {
  export interface SharedProps extends InferSharedProps<typeof inertiaConfig> {
    user: User | undefined
    subdomainUser: User | undefined
    [key: string]: any
  }
}
