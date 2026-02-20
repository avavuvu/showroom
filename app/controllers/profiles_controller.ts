import type { HttpContext } from '@adonisjs/core/http'
import { updateProfileValidator } from '#validators/profile'

import User from '#models/user'
import ImageService from '#services/image_service'

export default class ProfilesController {
    async index({ inertia }: HttpContext) {
        return inertia.render("user/profile", {
            defaultProfileImages: User.defaultProfileImageUrls
        })
    }

    async update({ request, auth, response }: HttpContext) {
        const user = auth.user!
        const payload = await request.validateUsing(updateProfileValidator, {
            meta: { userId: user.id }
        })

        user.merge(payload)
        await user.save()

        return response.redirect().back()
    }

    async updateProfileImage({ request, response, auth }: HttpContext) {
        const user = auth.getUserOrFail()

        const image = request.file('avatar', {
            size: '5mb',
            extnames: ['jpg', 'jpeg', 'png', 'webp'],
        })

        if (!image) return response.badRequest({ error: 'No image provided' })
        if (!image.isValid) return response.badRequest({ errors: image.errors })

        try {
            const { key, url } = await ImageService.storeProfilePicture(image, user.id)

            await user.merge({ profileImageUrl: key }).save() // store just the key, not the URL

            return { url, key }
        } catch (error) {
            console.error('Avatar upload error:', error)
            return response.internalServerError({ error: 'Failed to upload avatar' })
        }
    }
}