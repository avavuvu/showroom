import type { HttpContext } from '@adonisjs/core/http'
import vine from '@vinejs/vine'
import Newsletter from '#models/newsletter'
import User from '#models/user'
import { UserService } from '#services/user_service'


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

  async comment({ params }: HttpContext) {
    const validator = vine.compile(
      vine.object({
        content: vine.string(),
      })
    )


  }

  async edit({ params, inertia, auth }: HttpContext) {
    const user = auth.user!
    const newsletter = await this.findNewsletter(user, params.id)

    return inertia.render('newsletters/edit', {
      newsletter: newsletter,
    })
  }

  async update({ request, response, auth, params }: HttpContext) {
    const user = auth.user!
    const newsletter = await this.findNewsletter(user, params.id)

    const validator = vine.compile(
      vine.object({
        title: vine.string().trim().optional(),
        slug: vine.string().trim().optional(),
        subtitle: vine.string().trim().optional().nullable(),
        content: vine.string(),
      })
    )

    const payload = await request.validateUsing(validator)

    newsletter.merge({
      title: payload.title,
      slug: payload.slug,
      subtitle: payload.subtitle,
      content: payload.content,
    })

    await newsletter.save()

    return response.redirect().back()
  }

  async preview({ params, inertia, auth, view }: HttpContext) {
    const user = auth.user!
    const newsletter = await this.findNewsletter(user, params.id)

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

  async send({ params, inertia, auth }: HttpContext) {
    const user = auth.user!
    const newsletter = await this.findNewsletter(user, params.id)

    // just set it to sent, don't actually send for now
    newsletter.merge({
      sent: true
    })
    await newsletter.save()

    const url = new URL(
      `/posts/${newsletter.slug}`,
      UserService.getUserDomainUrl(user.username!))

    return inertia.location(url.toString())
  }

  private async findNewsletter(user: User, id: string) {
    const newsletter = await Newsletter.query()
      .where('id', id)
      .where('user_id', user.id)
      .firstOrFail()

    return newsletter
  }
}
