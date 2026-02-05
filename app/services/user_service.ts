import app from "@adonisjs/core/services/app";

export class UserService {
  static async getUserUrl(slug: string) {
    return app.makeURL(`/users/${slug}`)
  }
}
