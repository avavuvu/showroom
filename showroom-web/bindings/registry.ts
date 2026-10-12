import type { Registry } from "@bq/components/src/setups";
import type * as setups from "./setups";

export const registry: Registry = {
    "color-input": {
        tags: ["details"],
        refs: ["customColor", "hex", "preset"],
        many: ["preset"],
        handlers: ["change", "pick"],
        load: () => import("../src/components/theme_form/color_input/index").then((module) => module.colorInput satisfies setups.ColorInput),
    },
    "color-override": {
        tags: ["div"],
        refs: ["hex", "picker", "reset", "toggle"],
        many: [],
        handlers: ["change", "clear"],
        load: () => import("../src/components/theme_form/style_overrides/index").then((module) => module.colorOverride satisfies setups.ColorOverride),
    },
    "dirty-form": {
        tags: ["form"],
        refs: [],
        many: [],
        handlers: ["leave", "reset", "restore", "submit", "update"],
        load: () => import("../src/components/dirty_form/index").then((module) => module.dirtyForm satisfies setups.DirtyForm),
    },
    "dropdown": {
        tags: ["details"],
        refs: [],
        many: [],
        handlers: ["close", "outside"],
        load: () => import("../src/components/dropdown/index").then((module) => module.dropdown satisfies setups.Dropdown),
    },
    "editor-link-bar": {
        tags: ["div"],
        refs: ["open", "remove", "text", "url"],
        many: [],
        handlers: ["apply", "cancel", "clearProblem", "follow", "keepFocus", "leave", "start", "unlink"],
        load: () => import("../src/components/editor/link_bar/index").then((module) => module.editorLinkBar satisfies setups.EditorLinkBar),
    },
    "newsletter-editor": {
        tags: ["div"],
        refs: ["blockButton", "blockCurrent", "blockOption", "blockOptions", "body", "command", "fileInput", "imageFrame", "moreButton", "moreOptions", "saveActions", "status", "subtitle", "surface", "title", "toolbar", "uploadPlaceholder"],
        many: ["blockOption", "command"],
        handlers: ["filesPicked", "guard", "leave", "markDirty", "overwrite", "reload", "save", "toBody", "toSubtitle"],
        load: () => import("../src/components/editor/index").then((module) => module.newsletterEditor satisfies setups.NewsletterEditor),
    },
    "password-toggle": {
        tags: ["div"],
        refs: [],
        many: [],
        handlers: ["toggle"],
        load: () => import("@bq/components/src/input/index").then((module) => module.passwordToggle satisfies setups.PasswordToggle),
    },
    "theme-preview": {
        tags: ["form"],
        refs: [],
        many: [],
        handlers: ["schedule"],
        load: () => import("../src/components/theme_form/index").then((module) => module.themePreview satisfies setups.ThemePreview),
    },
};
