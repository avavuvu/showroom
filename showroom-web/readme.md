
## Tech Stack
I have dubbed in Ava's Max Stack

- Backend: Rust Axum
- Templating: Maud
- Islands: Vue
- Client-side Interactivity: Alpine.js
- Server-side Interactivity: HTMX

## To do:
- [x] edit page fetch
- [x] sending pages
- [x] creating a new newsletter
- [x] caching pages
- [x] header on every page
- [x] lander
- [x] markdown API
- [x] styling
- [ ] Dashboard styling
- [x] Disable HTMX/Islands on pages that dont need it
- [x] Remove create account page on prod
- [x] Figure out why fake users are not 404ing
    - [x] Enabble JS on user pages
- [x] Monorepo conversion
- [ ] Webhooks

### Features
- [ ] Subscribe embed
- [x] Image CDN (cloudinary?)
- [ ] Comments
- [ ] Webrings

### Bug fixes
- [x] Why does the profile page hang on load?
- [ ] Fix markdown conversion errors
    - [ ] Create test files
- [ ] Fix broken email links
- [ ] 404 pages don't link correctly
- [ ] Remove tower reload

### Profile pages
- [ ] Visual themes
    - [ ] Theme editor
- [ ] Profile descriptions
- [ ] Names beyond usernames
- [ ] Custom headers
- [ ] Profile link trees
- [ ] Hero images
- [ ] Main loading 

### Preparing for launch
- [ ] Banned account names
- [ ] Social media sign in
- [ ] Payments
#### SEO
    - [ ] Search Console

### The editor
- [x] Improve link editor
- [ ] Add proper loading animation to edit
- [ ] Add Placeholder
- [ ] List margins (and other inconsistencies with email)
- [ ] Code displays really weird on email

#### Keybinds
- [ ] Keyboard shortcuts
    - [x] Ctrl+K -> link menu
    - [x] Ctrl+Click on a link -> Open the link
 
## Migrations
`cargo run -p migration --bin migration -- up
