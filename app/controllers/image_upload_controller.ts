import type { HttpContext } from '@adonisjs/core/http'
import drive from '@adonisjs/drive/services/main'

export default class ImageUploadController {
  async store({ request, response }: HttpContext) {
    const image = request.file('image', {
      size: '2mb',
      extnames: ['jpg', 'png', 'jpeg', 'webp'],
    })

    if (!image) {
      return response.badRequest({ error: 'Image is missing' })
    }

    if (image.hasErrors) {
      return response.badRequest(image.errors)
    }

    const key = `uploads/${Date.now()}-${image.clientName}`
    await image.moveToDisk(key)
    
    // In a real app we might need to generate a full URL
    // For local driver, we can serve it via a static route or similar
    // Assuming 'uploads' is publicly accessible or mapped
    
    // For now, let's return the URL that can be used to access it.
    // Since we are using the default local driver which uses storage/app, 
    // we normally need a route to serve these or symlink.
    // However, Adonis Drive has getUrl method.
    
    const url = await drive.use().getUrl(key)
    return { url }
  }
}
