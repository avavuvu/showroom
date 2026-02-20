import type { HttpContext } from '@adonisjs/core/http'
import type { NextFn } from '@adonisjs/core/types/http'
import User from '#models/user'

export default class FindSubdomainUserMiddleware {
    async handle(ctx: HttpContext, next: NextFn) {
        const username = ctx.subdomains.username

        // TODO: this shouldn't redirect to 404
        if (!username) {
            return ctx.response.redirect('/404')
        }

        const user = await User.findBy('username', username)

        if (!user) {
            return ctx.response.redirect('/404')
        }

        ctx.subdomainUser = user

        await next()
    }
}
