import type { HttpContext } from '@adonisjs/core/http'
import Scrap from '#models/scrap'
import vine from '@vinejs/vine'

export default class ScrapsController {
    async create({ response, auth }: HttpContext) {
        const user = auth.user!

        const scrap = await Scrap.create({
            userId: user.id,
            content: '',
        })

        return response.redirect().toRoute('scrap.edit', { id: scrap.id })
    }

    async edit({ params, inertia, auth }: HttpContext) {
        const user = auth.user!
        const scrap = await Scrap.query()
            .where('id', params.id)
            .where('user_id', user.id)
            .firstOrFail()

        return inertia.render('user/scrap', {
            scrap
        })
    }

    async update({ request, response, auth, params }: HttpContext) {
        const user = auth.user!
        const scrap = await Scrap.query()
            .where('id', params.id)
            .where('user_id', user.id)
            .firstOrFail()

        const validator = vine.compile(
            vine.object({
                content: vine.string(),
            })
        )

        const payload = await request.validateUsing(validator)

        scrap.merge({
            content: payload.content,
        })

        await scrap.save()

        return response.redirect().back()
    }
}