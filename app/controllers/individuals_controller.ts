import Newsletter from '#models/newsletter'
import Subscriber from '#models/subscriber'
import type { HttpContext } from '@adonisjs/core/http'
import vine from '@vinejs/vine'
import mail from "@adonisjs/mail/services/main"
import { UserService } from '#services/user_service'

export default class IndividualsController {

  async show({ inertia, subdomainUser, auth }: HttpContext) {
    return inertia.render("user/archive", {
      user: auth.user,
      newsletters: inertia.defer(async () => {
        const newsletters = await Newsletter.query()
          .where('userId', subdomainUser!.id)
          .where('sent', true)
          .orderBy('createdAt', 'desc')

        return newsletters
      })
    })
  }

  async show_post({ params, subdomainUser, inertia }: HttpContext) {
    const slug = params.slug

    const newsletter = await Newsletter.query()
      .where('slug', slug)
      .where('userId', subdomainUser!.id)
      .where('sent', true)
      .firstOrFail()

    return inertia.render("newsletters/post", {
      newsletter: newsletter
    })
  }

  async subscribe({ subdomainUser, response, request }: HttpContext) {
    const validator = vine.compile(
      vine.object({
        email: vine.string().email().normalizeEmail(),
        name: vine.string().maxLength(24).optional()
      })
    )

    const { email, name } = await request.validateUsing(validator)

    const subscriber = await Subscriber.firstOrCreate(
      { email, userId: subdomainUser!.id },
      { email, name, userId: subdomainUser!.id }
    )

    let confirmUrl = new URL(
      "/confirm",
      UserService.getUserDomainUrl(subdomainUser!.username!)
    )

    confirmUrl.searchParams.set("email", email)
    confirmUrl.searchParams.set("token", subscriber.token)

    // email
    await mail.send(message => {
      message.to(email)
        .from(`${subdomainUser!.username}@showroom.you`)
        .subject('Verify your email address')
        .html(`
          <a href=${confirmUrl.toString()}>Confirm</a> your subscription to ${subdomainUser?.username}
          `)
    })

    return response.redirect().withQs({ name: subscriber.name }).toPath('/subscribe/redirect')
  }

  async subscribe_index({ inertia }: HttpContext) {
    return inertia.render("user/subscribe", {})
  }

  async subscribe_redirect({ inertia, request }: HttpContext) {
    return inertia.render("user/subscribe_redirect", {
      name: request.qs().name
    })
  }

  async confirm({ request, inertia }: HttpContext) {
    const { email, token }: { email?: string, token?: string } = request.qs()

    if (!email || !token) {
      return inertia.render("user/subscribe_confirm", {
        success: false,
      })
    }

    const subscriber = await Subscriber.query()
      .where('email', email)
      .andWhere('token', token)
      .preload('user')
      .first()

    if (!subscriber) {
      return inertia.render("user/subscribe_confirm", {
        success: false,
      })
    }

    subscriber.verified = true
    await subscriber.save()

    // in future, we will grab the username, a header image, and their latest post.
    return inertia.render("user/subscribe_confirm", {
      success: true,
      name: subscriber.name,
      username: subscriber.user.username
    })
  }

}