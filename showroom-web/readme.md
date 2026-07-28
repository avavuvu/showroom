
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
- [ ] Monorepo conversion

### Features
- [ ] Subscribe embed
- [ ] Image CDN (cloudinary?)
- [ ] Comments
- [ ] Webrings

### Bug fixes
- [ ] Why does the profile page hang on load?
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

### Preparing for launch
- [ ] Banned account names
- [ ] Social media sign in
- [ ] Payments
#### SEO
    - [ ] Search Console

### The editor
- [ ] Improve link editor
- [ ] Keyboard shortcuts
    - [ ] Ctrl+K -> link menu
- [ ] Add proper loading animation to edit
- [ ] Add Placeholder
- [ ] List margins (and other inconsistencies with email)
- [ ] Code displays really weird on email
 
## Migrations
`cargo run -p migration --bin migration -- up
