import Newsletter from '#models/newsletter'
import Scrap from '#models/scrap'
import type { HttpContext } from '@adonisjs/core/http'

export default class DashboardController {
    async show(context: HttpContext) {
        return context.inertia.render("user/dashboard", {
            newsletters: context.inertia.defer(async () => {
                const allNewsletters = await Newsletter.query()
                    .where('userId', context.auth.user!.id)
                    .orderBy('updatedAt', 'desc')

                return {
                    sent: allNewsletters.filter(n => n.sent),
                    unsent: allNewsletters.filter(n => !n.sent)
                }
            }),
            scraps: context.inertia.defer(async () => {
                const scraps = await Scrap.query()
                    .where('userId', context.auth.user!.id)
                    .orderBy('updatedAt', 'desc')
                return scraps
            })
        })
    }
}