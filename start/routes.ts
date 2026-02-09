/*
|--------------------------------------------------------------------------
| Routes file
|--------------------------------------------------------------------------
|
| The routes file is used for defining the HTTP routes.
|
*/

import router from '@adonisjs/core/services/router'
import { middleware } from './kernel.js'
import env from '#start/env'
import User from '#models/user'
import Scrap from '#models/newsletter'

const LogoutController = () => import('#controllers/auth/logout_controller')
const LoginController = () => import('#controllers/auth/login_controller')
const RegisterController = () => import('#controllers/auth/register_controller')
const NewslettersController = () => import('#controllers/newsletters_controller')
const ImageUploadController = () => import('#controllers/image_upload_controller')
const DashboardController = () => import('#controllers/dashboard_controller')
const ScrapsController = () => import('#controllers/scraps_controller')

router.get("/", async (context) => {
  return context.inertia.render("home")
}).as("home")

router.group(() => {
  router.get("/", async ({ subdomains, inertia, response }) => {

    const username = subdomains.username

    const user = await User.findBy("username", username)

    if (!user) {
      return response.redirect("/404")
    }

    return inertia.render("user/archive", {
      user,
      newsletters: inertia.defer(async () => {
        const newsletters = await Scrap.query()
          .where('userId', user.id)
          .orderBy('createdAt', 'desc')

        return newsletters
      })
    })

  })
}).domain(
  env.get("NODE_ENV") === "development"
    ? ":username.localtest.me" //localtest is a publically available domain that just redirects to localhost
    : ":username.showroom.you"
)

router.get("/dashboard", [DashboardController, "show"]).as("dashboard").use(middleware.auth())

router.post("/scrap", [ScrapsController, "create"]).as("scrap.create").use(middleware.auth())
router.get("/scrap/:id", [ScrapsController, "edit"]).as("scrap.edit").use(middleware.auth())
router.put("/scrap/:id", [ScrapsController, "update"]).as("scrap.update").use(middleware.auth())

router.get("/publish/:id", [NewslettersController, "edit"]).as("publish.edit").use(middleware.auth())
router.put("/publish/:id", [NewslettersController, "update"]).as("publish.update").use(middleware.auth())
router.post("/newsletters", [NewslettersController, "create"]).as("newsletters.create").use(middleware.auth())
router.post("/images", [ImageUploadController, "store"]).as("images.store").use(middleware.auth())

router.group(() => {
  router.get("/register", [RegisterController, "show"]).as("register.show").use(middleware.guest())
  router.post("/register", [RegisterController, "store"]).as("register.store").use(middleware.guest())

  router.get("/login", [LoginController, "show"]).as("login.show").use(middleware.guest())
  router.post("/login", [LoginController, "store"]).as("login.store").use(middleware.guest())

  router.post("/logout", [LogoutController, "handle"]).as("logout").use(middleware.auth())
}).as("auth")
