import type { HttpContext } from '@adonisjs/core/http'
import drive from '@adonisjs/drive/services/main'

export default class UploadsController {
  async store({ request, response }: HttpContext) {
    const image = request.file('image', {
      size: '5mb',
      extnames: ['jpg', 'jpeg', 'png', 'webp', 'gif'],
    })

    if (!image) {
      return response.badRequest({ error: 'No image provided' })
    }

    if (!image.isValid) {
      return response.badRequest({ errors: image.errors })
    }

    try {
      const fileName = `${Date.now()}-${image.clientName}`

      await image.moveToDisk(fileName, 's3')

      const url = await drive.use('s3').getUrl(fileName)

      return { url, fileName }
    } catch (error) {
      console.error('Upload error:', error)
      return response.internalServerError({
        error: 'Failed to upload',
        details: error.message
      })
    }
  }
}