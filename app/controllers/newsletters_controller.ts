import type { HttpContext } from '@adonisjs/core/http'
import vine from '@vinejs/vine'
import Newsletter from '#models/newsletter'
import drive from '@adonisjs/drive/services/main'
import app from '@adonisjs/core/services/app'
import fs from 'node:fs/promises'

export default class NewslettersController {
  async create({ response, auth }: HttpContext) {
    const user = auth.user!

    const newsletter = await Newsletter.create({
      userId: user.id,
      title: 'Untitled Newsletter',
      content: '',
    })

    return response.redirect().toRoute('publish.edit', { id: newsletter.id })
  }

  async edit({ params, inertia, auth }: HttpContext) {
    const user = auth.user!
    const newsletter = await Newsletter.query()
      .where('id', params.id)
      .where('user_id', user.id)
      .firstOrFail()

    return inertia.render('user/publish', {
      newsletter: newsletter
    })
  }

  async update({ request, response, auth, params }: HttpContext) {
    const user = auth.user!
    const newsletter = await Newsletter.query()
      .where('id', params.id)
      .where('user_id', user.id)
      .firstOrFail()

    const validator = vine.compile(
      vine.object({
        title: vine.string().trim(),
        content: vine.string(),
      })
    )

    const payload = await request.validateUsing(validator)

    newsletter.merge({
      title: payload.title,
      content: payload.content,
    })

    await newsletter.save()

    return response.redirect().back()
  }

  async test({ response }: HttpContext) {
    const filePath = 'storage/uploads/1770191101531-Photo on 3-9-2025 at 1.29 PM.jpg'
    const absolutePath = app.makePath(filePath)

    try {
      const fileContent = await fs.readFile(absolutePath)
      const fileName = `tests/${Date.now()}-test-image.jpg`
      await drive.use('s3').put(fileName, fileContent)
      const url = await drive.use('s3').getUrl(fileName)
      return response.ok({ url, fileName })
    } catch (error) {
      console.error(error)
      return response.internalServerError({ message: 'S3 test failed', error: error.message })
    }
  }
}
