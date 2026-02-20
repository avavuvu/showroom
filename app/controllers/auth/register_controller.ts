import User from '#models/user'
import { quickRegisterValidator, registerValidator } from '#validators/auth'
import type { HttpContext } from '@adonisjs/core/http'

export default class RegisterController {

  async show(context: HttpContext) {
    return context.inertia.render("auth/register")
  }

  async quick_register({ request, auth, response }: HttpContext) {
    console.log({ request })

    const data = request.only([
      "username",
      "email",
    ])

    const validatedData = await quickRegisterValidator.validate(data)

    const user = await User.create({ ...validatedData, password: "password" })

    await auth.use("web").login(user)

    return response.redirect().back()
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
