import type { HttpContext } from '@adonisjs/core/http'

export default class ProfilesController {
    async index({ inertia }: HttpContext) {
        return inertia.render("user/profile")
    }
}