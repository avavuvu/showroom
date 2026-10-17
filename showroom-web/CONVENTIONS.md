# TypeScript conventions

These apply to all TS in `src/**`, `resources/**` and `vite.config.ts`. Run `bun run lint` to check and `bun run lint:fix` to autofix what can be fixed. `bindings/**` is generated and is not linted.

Formatting: 4-space indent, double quotes, trailing commas in multiline literals, `1tbs` braces. All of it is enforced by `@stylistic` in `eslint.config.js`.

## 1. Always use braces (lint: `curly`)

Avoid:

```ts
if (!view) return
if (isOpen) close(); else open();
```

Prefer:

```ts
if (!view) {
    return
}
if (isOpen) {
    close()
} else {
    open()
}
```

## 2. No semicolons unless required (lint: `@stylistic/semi`, `@stylistic/member-delimiter-style`)

This covers interface and type members too. Single-line type literals use commas: `{ from: number, to: number }`. Only keep a semicolon where a line would otherwise join the previous one, i.e. a line starting with `(`, `[` or a template literal:

```ts
const items = getItems()
;[first, second].forEach(focusItem)
```

## 3. `Array<T>` over `T[]` (lint: `@typescript-eslint/array-type`)

```ts
const blocks: Array<Node> = []
const update = (decorations: ReadonlyArray<Decoration>) => {}
```

## 4. Arrow functions (lint: `func-style`, `prefer-arrow-callback`, `no-restricted-syntax`)

Use arrow functions for top-level functions, callbacks and object members. That includes setup handlers and ProseMirror spec/prop functions.

```ts
export const findAncestor = (state: EditorState, predicate: (node: Node) => boolean) => {}

return {
    close: (event) => {},
}
```

Class methods are fine. Other non-arrow functions are fine only with a real reason: a function that uses its own `this`, a generator, or overload signatures. Hoisting is not a reason. Define helpers before the code that uses them.

## 5. Functions start with a verb (by hand)

`renderCard`, `getAttrs`, `createView`, `findInsertionPoint`, `setFieldValue`, `isAllowedHref`.

Exceptions:

- **Factories named after what they produce**, when that is the ProseMirror idiom: `musicPlugin()`, `linkPlugin()`, `placeholders()`, `trailingParagraph()`, `imageView()`, `blockView()`.
- **Names set by someone else:** framework or library hooks (`handlePaste`, `appendTransaction`, `update`, `destroy`, `disconnect`) and setup handler keys. Setup handler keys (`outside`, `toBody`, `filesPicked`) and exported setup functions (`menu`, `dropdown`, `newsletterEditor`) come from Rust via `bindings/registry.ts` and `bindings/setups.ts`, so never rename them in TS.

## 6. Booleans read as questions (lint: `@typescript-eslint/naming-convention`)

Boolean variables, parameters, class properties, getters and type properties start with `is`, `has`, `should`, `can`, `was` or `did`:

```ts
let isSubmitting = false
get hasUnsavedChanges(): boolean {}
const holdUnload = (event: BeforeUnloadEvent, hasUnsavedChanges: boolean) => {}
```

External shapes are exempt: object literals passed to APIs (`{ once: true }`, `atom: true`, `draggable: true`), DOM properties (`hidden`, `disabled`, `multiple`), destructured fields (`const { empty } = state.selection`), and JSON or attr fields shared with Rust.

## 7. Long, descriptive names (lint: `id-denylist` for the common abbreviations)

| Avoid | Prefer |
| --- | --- |
| `e`, `ev`, `evt` | `event` |
| `el`, `elem` | `element` |
| `err` | `error` |
| `ctx` | `context` |
| `tr`, `txn` | `transaction` |
| `btn` | `button` |
| `cb` | `callback` |
| `idx`, `i` | `index` |
| `msg`, `res`, `req`, `val`, `tmp`, `str`, `obj`, `arr` | the full word |

ProseMirror's `state.tr` is external API, so access it freely, but name the local `transaction`:

```ts
const transaction = state.tr
transaction.insert(pos, node)
dispatch(transaction.scrollIntoView())
```

Established short names are fine:

- Coordinates: `x`, `y`.
- Common words: `id`, `url`, `href`, `src`.
- WebGL: `gl`.
- ProseMirror: `pos`, `doc`, `attrs`, and resolved positions like `$pos` and `$from`.
- Sort comparators: `a`, `b`.
