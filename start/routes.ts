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
import env from './env.js'

const LogoutController = () => import('#controllers/auth/logout_controller')
const LoginController = () => import('#controllers/auth/login_controller')
const RegisterController = () => import('#controllers/auth/register_controller')
const NewslettersController = () => import('#controllers/newsletters_controller')
const ImageUploadController = () => import('#controllers/image_upload_controller')
const DashboardController = () => import('#controllers/dashboard_controller')
const ScrapsController = () => import('#controllers/scraps_controller')
const IndividualsController = () => import('#controllers/individuals_controller')
const ProfilesController = () => import('#controllers/profiles_controller')

router.get("/", async (context) => {
  return context.inertia.render("home")
}).as("home").use(middleware.silentAuth())

router.group(() => {
  router.get("/subscribe", [IndividualsController, "subscribe_index"]).as("subscribe.index")
  router.get("/", [IndividualsController, "show"]).as("archive.show")
  router.post("/subscribe", [IndividualsController, "subscribe"]).as("subscribe.store")
  router.get("/subscribe/redirect", [IndividualsController, "subscribe_redirect"]).as("subscribe.redirect")
  router.get("/confirm", [IndividualsController, "confirm"]).as("subscribe.confirm")
  router.get("/posts/:slug", [IndividualsController, "show_post"]).as("posts.index")
}).domain(
  env.get("NODE_ENV") === "development"
    ? ":username.localtest.me" //localtest is a publically available domain that just redirects to localhost
    : ":username.showroom.you"
).use([middleware.silentAuth(), middleware.findSubdomainUser()])

router.get("/dashboard", [DashboardController, "show"]).as("dashboard").use(middleware.auth())
router.get("/profile", [ProfilesController, "index"]).as("profile").use(middleware.auth())
router.put("/profile", [ProfilesController, "update"]).as("profile.update").use(middleware.auth())
router.post("/profile/image", [ProfilesController, "updateProfileImage"]).as("profile.image").use(middleware.auth())

router.post("/scrap", [ScrapsController, "create"]).as("scrap.create").use(middleware.auth())
router.get("/scrap/:id", [ScrapsController, "edit"]).as("scrap.edit").use(middleware.auth())
router.put("/scrap/:id", [ScrapsController, "update"]).as("scrap.update").use(middleware.auth())

router.get("/publish/:id", [NewslettersController, "edit"]).as("publish.edit").use(middleware.auth())
router.put("/publish/:id", [NewslettersController, "update"]).as("publish.update").use(middleware.auth())
router.get("/publish/:id/preview", [NewslettersController, "preview"]).as("publish.preview").use(middleware.auth())
router.post("/publish/:id/send", [NewslettersController, "send"]).as("publish.send").use(middleware.auth())
router.post("/newsletters", [NewslettersController, "create"]).as("newsletters.create").use(middleware.auth())
router.post("/images", [ImageUploadController, "store"]).as("images.store").use(middleware.auth())

router.group(() => {
  router.get("/register", [RegisterController, "show"]).as("register.show").use(middleware.guest())
  router.post("/register", [RegisterController, "store"]).as("register.store").use(middleware.guest())

  router.post("/quick", [RegisterController, "quick_register"]).as("register.quick").use(middleware.guest())

  router.get("/login", [LoginController, "show"]).as("login.show").use(middleware.guest())
  router.post("/login", [LoginController, "store"]).as("login.store").use(middleware.guest())

  router.get("/logout", [LogoutController, "handle"]).as("logout").use(middleware.auth())
}).as("auth")
