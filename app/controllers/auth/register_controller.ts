import User from '#models/user'
import { registerValidator } from '#validators/auth'
import type { HttpContext } from '@adonisjs/core/http'

export default class RegisterController {

  async show(context: HttpContext ) {
    return context.inertia.render("auth/register")
  }

  async store({ request, response, auth }: HttpContext) {
    const data = request.only([
      "fullName",
      "username",
      "email",
      "password"
    ])

    const validatedData = await registerValidator.validate(data)

    const user = await User.create(validatedData)

    await auth.use("web").login(user)

    return response.redirect().toRoute("home")
  }

}
