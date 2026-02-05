import type { HttpContext } from '@adonisjs/core/http'
import vine from '@vinejs/vine'
import Newsletter from '#models/newsletter'

export default class NewslettersController {
  async create({ response, auth }: HttpContext) {
    const user = auth.user!
    
    // Create a blank newsletter immediately
    const newsletter = await Newsletter.create({
      userId: user.id,
      title: 'Untitled Newsletter',
      content: '', // Start with empty content
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
}
