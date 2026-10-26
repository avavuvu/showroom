
## Boutique
This project uses boutique, a rust ssr library.

## Creating a new account.
heroku run -a show-room ./target/release/create_user a@example.com handle


## To do:
- [ ] RSS
- [ ] Sitemap
- [ ] Webhooks

## To do, Monday 26 Oct
- [x] investigate content being overridden
- [ ] Style presets
- [ ] Disabling wood trim
- [ ] Better OG images and favicon

### Features
- [ ] Subscribe embed
- [x] Image CDN (cloudinary?)
- [ ] Comments
- [ ] youtube support
- [ ] Scraps
- [ ] Video / audio support
- [ ] Music support (?)
- [ ] Monthly (/ custom) reminders
- [ ] Webrings
- [ ] New icons
- [x] Woodgrain
- [ ] User generated templates
- [ ] Layouts from BQ components
- [ ] Extensive styling of user index page
- [ ] Index page preview

### Bug fixes
- [x] Why does the profile page hang on load?
- [ ] Fix markdown conversion errors
- [ ] Fix broken email links
- [ ] 404 pages don't link correctly
- [x] Remove tower reload
- [ ] Awkward interaction between heading and hr

### Profile pages
- [ ] Names beyond usernames
- [ ] Profile link trees
- [x] Hero images
- [ ] Main loading 

### Preparing for launch
- [x] Banned account names
- [ ] Social media sign in
- [ ] Payments
- [ ] Patreon sign in

#### SEO
- [ ] Search Console

### The editor
- [x] Improve link editor
- [ ] Add proper loading animation to edit
- [x] Add Placeholder
- [x] List margins (and other inconsistencies with email)
- [?] Code displays really weird on email
- [x] Move greeting into editor
- [x] {{name}} input
 
## Migrations
```
cargo run -p migration --bin migration -- up
sea-orm-cli migrate generate <name> -d migration
cargo run -p migration --bin seed --features seed
```
