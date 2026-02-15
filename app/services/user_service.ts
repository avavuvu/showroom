import app from "@adonisjs/core/services/app";
import env from "#start/env";

export class UserService {
  static async getUserUrl(slug: string) {
    return app.makeURL(`/users/${slug}`)
  }

  static getUserDomainUrl(username: string) {
    const domain = env.get("NODE_ENV") === "development"
      ? `${username}.localtest.me:3333`
      : `${username}.showroom.you`

    const protocol = env.get("NODE_ENV") === "development" ? "http" : "https"

    return `${protocol}://${domain}`
  }

}