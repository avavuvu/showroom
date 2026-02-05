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
const LogoutController = () => import('#controllers/auth/logout_controller')
const LoginController = () => import('#controllers/auth/login_controller')
const RegisterController = () => import('#controllers/auth/register_controller')
const NewslettersController = () => import('#controllers/newsletters_controller')
const ImageUploadController = () => import('#controllers/image_upload_controller')
import Newsletter from '#models/newsletter'


router.get("/", async (context) => {
  return context.inertia.render("home")
}).as("home")


router.get("/dashboard", async (context) => {
  return context.inertia.render("user/dashboard", {
    user: context.auth.user,
    newsletters: context.inertia.defer(async () => {
      await new Promise(resolve => setTimeout(resolve, 500))

      const newsletters = await Newsletter.query()
        .where('userId', context.auth.user!.id)
        .orderBy('createdAt', 'desc')

      return newsletters
    })
  })
}).as("dashboard").use(middleware.auth())

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
}).prefix("/auth").as("auth")
