import type { Setup } from "@bq/components/src/setups";

export type ColorInput = Setup<"details", { customColor: "input"; hex: "span"; preset: ("button")[] }>;
export type ColorOverride = Setup<"div", { hex: "span"; picker: "input"; reset: "button"; toggle: "input" }>;
export type DirtyForm = Setup<"form">;
export type Dropdown = Setup<"details">;
export type NewsletterEditor = Setup<"div", { blockButton: "button"; blockCurrent: "span"; blockOption: ("button")[]; blockOptions: "div"; body: "div"; command: ("button")[]; fileInput: "input"; imageFrame: "template"; leave: (string)[]; linkBar: "div"; linkInput: "input"; linkOpen: "a"; linkRemove: "button"; linkText: "input"; moreButton: "button"; moreOptions: "div"; overwrite: string; reload: string; saveActions: "span"; status: "output"; subtitle: "input"; surface: "div"; title: "input"; toolbar: "div"; uploadPlaceholder: "template" }>;
export type PasswordToggle = Setup<"div", { toggle: "button" }>;
export type ThemePreview = Setup<"form">;
