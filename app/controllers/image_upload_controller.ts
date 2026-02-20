import type { HttpContext } from '@adonisjs/core/http'
import ImageService from '#services/image_service'

export default class UploadsController {
  async store({ request, response }: HttpContext) {
    const image = request.file('image', {
      size: '5mb',
      extnames: ['jpg', 'jpeg', 'png', 'webp', 'gif'],
    })

    if (!image) return response.badRequest({ error: 'No image provided' })
    if (!image.isValid) return response.badRequest({ errors: image.errors })

    try {
      const { key, url } = await ImageService.storeImage(image)
      return { url, key }
    } catch (error) {
      console.error('Upload error:', error)
      return response.internalServerError({ error: 'Failed to upload', details: error.message })
    }
  }
}