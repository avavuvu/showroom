import type { Setup } from "@bq/components/src/setups";

export type ColorInput = Setup<"details", { customColor: "input"; hex: "span"; preset: ("button")[] }>;
export type ColorOverride = Setup<"div", { hex: "span"; picker: "input"; reset: "button"; toggle: "input" }>;
export type DirtyForm = Setup<"form">;
export type NewsletterEditor = Setup<"div">;
export type PasswordToggle = Setup<"div", { toggle: "button" }>;
export type SaveStatus = Setup<"output">;
export type ThemePreview = Setup<"form">;
