import type { Registry } from "@bq/components/src/setups";
import type * as setups from "./setups";

export const registry: Registry = {
    "color-input": {
        tags: ["details"],
        many: ["preset"],
        load: () => import("../src/components/color_input/index").then((module) => module.colorInput satisfies setups.ColorInput),
    },
    "color-override": {
        tags: ["div"],
        many: [],
        load: () => import("../src/components/style_overrides/index").then((module) => module.colorOverride satisfies setups.ColorOverride),
    },
    "dirty-form": {
        tags: ["form"],
        many: [],
        load: () => import("../src/components/dirty_form/index").then((module) => module.dirtyForm satisfies setups.DirtyForm),
    },
    "dropdown": {
        tags: ["details"],
        many: [],
        load: () => import("../src/components/dropdown/index").then((module) => module.dropdown satisfies setups.Dropdown),
    },
    "newsletter-editor": {
        tags: ["div"],
        many: ["blockOption", "command", "leave"],
        load: () => import("../src/components/editor/index").then((module) => module.newsletterEditor satisfies setups.NewsletterEditor),
    },
    "password-toggle": {
        tags: ["div"],
        many: [],
        load: () => import("@bq/components/src/input/index").then((module) => module.passwordToggle satisfies setups.PasswordToggle),
    },
    "theme-preview": {
        tags: ["form"],
        many: [],
        load: () => import("../src/components/color_input/index").then((module) => module.themePreview satisfies setups.ThemePreview),
    },
};
