# Showroom Monorepo
_Newsletters for people like you_

```
 How this email finds you.
   _____
  | O O |
  |  >  |
  | \_/ |
  \_____/
     |
  -------   _____ 
  |     |  |Ava's|
==|     |==|Email|
  |     |  \_____/
  =======
   || ||
   || ||
 __/   \__
```

## /showroom-web
Showroom fullstack backend/frontend.

CSS and JS are built by Vite into `showroom-web/public/build` with a manifest; pages reference entries by name (`Head::entry("site")`). Static files are served straight from `resources/static`. Shared component CSS comes from `bq_components` via the `@bq` alias.

## /migration
Database migrations/seeders crate

## boutique
Server/session library and shared components. Lives in its own repository at `../../dev/boutique` (path dependency).
