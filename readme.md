# Showroom
_Newsletters for people like you_

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
- [ ] Add proper loading animation to edit
- [ ] Add Placeholder
- [ ] Remove create account page on prod
- [ ] Figure out why fake users are not 404ing
    - [ ] Enabble JS on user pages
 
## Migrations
`cargo run -p migration --bin migration -- up
