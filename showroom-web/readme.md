
## Tech Stack
I have dubbed in Ava's Max Stack

- Backend: Rust Axum
- Templating: Maud + `boutique` extensions
- Islands: Vue
- Server-side Interactivity: HTMX
- Auth, layouts, and JS: `boutique` crate

## To do:
- [x] edit page fetch
- [x] sending pages
- [x] creating a new newsletter
- [x] caching pages
- [x] header on every page
- [x] lander
- [x] markdown API
- [x] styling
- [x] Dashboard styling
- [x] Disable HTMX/Islands on pages that dont need it
- [x] Remove create account page on prod
- [x] Figure out why fake users are not 404ing
    - [x] Enabble JS on user pages
- [x] Monorepo conversion
- [ ] RSS
- [ ] Sitemap
- [ ] Webhooks
- [x] convert to HTMX 4

### Features
- [ ] Subscribe embed
- [x] Image CDN (cloudinary?)
- [ ] Comments

### Bug fixes
- [x] Why does the profile page hang on load?
- [ ] Fix markdown conversion errors
    - [ ] Create test files
- [ ] Fix broken email links
- [ ] 404 pages don't link correctly
- [x] Remove tower reload

### Profile pages
- [x] Visual themes
    - [x] Theme editor
- [x] Profile descriptions
- [x] Profile pictures
- [ ] Names beyond usernames
- [ ] Custom headers
- [ ] Profile link trees
- [ ] Hero images
- [x] Main loading 

### Preparing for launch
- [ ] Banned account names
- [ ] Social media sign in
- [ ] Payments
#### SEO
    - [ ] Search Console

### The editor
- [x] Improve link editor
- [ ] Add proper loading animation to edit
- [x] Add Placeholder
- [x] List margins (and other inconsistencies with email)
- [ ] Code displays really weird on email

#### Keybinds
- [ ] Keyboard shortcuts
    - [x] Ctrl+K -> link menu
    - [x] Ctrl+Click on a link -> Open the link

### Blue sky future features
- [ ] Webrings

 
## Migrations
```
cargo run -p migration --bin migration -- up
sea-orm-cli migrate generate <name> -d migration
cargo run -p migration --bin seed --features seed
```
