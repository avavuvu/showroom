import type { HttpContext } from '@adonisjs/core/http'
import Scrap from '#models/scrap'
import vine from '@vinejs/vine'

export default class ScrapsController {
    async create({ response, auth }: HttpContext) {
        const user = auth.user!

        const scrap = await Scrap.create({
            userId: user.id,
            content: '',
            isSuccessful: false,
        })

        return response.redirect().toRoute('scrap.edit', { id: scrap.id })
    }

    async edit({ params, inertia, auth }: HttpContext) {
        const user = auth.user!
        const scrap = await Scrap.query()
            .where('id', params.id)
            .where('user_id', user.id)
            .firstOrFail()

        return inertia.render('scraps/edit', {
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
                isSuccessful: vine.boolean()
            })
        )

        const payload = await request.validateUsing(validator)

        scrap.merge({
            content: payload.content,
            isSuccessful: payload.isSuccessful
        })

        await scrap.save()

        return response.redirect().back()
    }
}