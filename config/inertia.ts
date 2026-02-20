import { defineConfig } from '@adonisjs/inertia'
import type { InferSharedProps } from '@adonisjs/inertia/types'

const inertiaConfig = defineConfig({
  /**
   * Path to the Edge view that will be used as the root view for Inertia responses
   */
  // TODO: move the preview conditional rendering here... https://docs.adonisjs.com/guides/views-and-templates/inertia
  rootView: 'inertia_layout',

  /**
   * Data that should be shared with all rendered pages
   */
  sharedData: {
    user: (context) => {
      const user = context.auth.user
      if (!user) return null
      return {
        ...user.serialize(),
        profileImageUrl: user.profileImageUrl ? ImageService.getUrl(user.profileImageUrl) : null,
      }
    },
    subdomainUser: (context) => {
      const user = context.subdomainUser
      console.log("test here")

      if (!user) return null
      return {
        ...user.serialize(),
        profileImageUrl: user.profileImageUrl ? ImageService.getUrl(user.profileImageUrl) : null,
      }
    },
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
import ImageService from '#services/image_service'

declare module '@adonisjs/inertia/types' {
  export interface SharedProps extends InferSharedProps<typeof inertiaConfig> {
    user: User | undefined
    subdomainUser: User | undefined
    [key: string]: any
  }
}
