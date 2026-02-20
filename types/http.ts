import type User from '#models/user'

declare module '@adonisjs/core/http' {
    interface HttpContext {
        subdomainUser?: User
    }
}
