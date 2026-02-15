import type { HttpContext } from '@adonisjs/core/http'
import vine from '@vinejs/vine'
import Newsletter from '#models/newsletter'

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

    return inertia.render('newsletters/edit', {
      newsletter: newsletter,
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
        title: vine.string().trim().optional(),
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

  async send({ params, inertia, auth, view }: HttpContext) {
    const user = auth.user!
    const newsletter = await Newsletter.query()
      .where('id', params.id)
      .where('user_id', user.id)
      .firstOrFail()

    const html = await view.render("emails/standard", {
      content: newsletter.content,
      username: user.username
    })

    // do not use default inertia template, because that has tailwind enabled, which doesnt work in email.
    return inertia.render('newsletters/preview', {
      newsletter,
      html,
      username: user.username,
      fullname: user.fullName,
    })
  }
}
