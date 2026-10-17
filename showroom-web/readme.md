
## Boutique
This project uses boutique, a rust ssr library.

## Creating a new account.
heroku run -a show-room ./target/release/create_user a@example.com handle


## To do:
- [ ] RSS
- [ ] Sitemap
- [ ] Webhooks

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
- [ ] Woodgrain
- [ ] User generated templates

### Bug fixes
- [x] Why does the profile page hang on load?
- [ ] Fix markdown conversion errors
- [ ] Fix broken email links
- [ ] 404 pages don't link correctly
- [x] Remove tower reload

### Profile pages
- [ ] Names beyond usernames
- [ ] Profile link trees
- [ ] Hero images
- [ ] Main loading 

### Preparing for launch
- [ ] Banned account names
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
- [ ] Move greeting into editor
- [ ] {{name}} input
 
## Migrations
```
cargo run -p migration --bin migration -- up
sea-orm-cli migrate generate <name> -d migration
cargo run -p migration --bin seed --features seed
```
