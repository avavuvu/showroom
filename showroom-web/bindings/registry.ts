import type { Registry } from "./boutique/setups";
import type * as setups from "./setups";

export const registry: Registry = {
    "ascii-background": {
        tags: ["div"],
        many: [],
        load: () => import("../src/components/ascii_background/index").then((module) => module.asciiBackground satisfies setups.AsciiBackground),
    },
    "color-input": {
        tags: ["details"],
        many: ["preset"],
        load: () => import("../src/components/theme_form/color_input/index").then((module) => module.colorInput satisfies setups.ColorInput),
    },
    "color-override": {
        tags: ["div"],
        many: [],
        load: () => import("../src/components/theme_form/style_overrides/index").then((module) => module.colorOverride satisfies setups.ColorOverride),
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
    "editor-link-bar": {
        tags: ["div"],
        many: [],
        load: () => import("../src/components/editor/link_bar/index").then((module) => module.editorLinkBar satisfies setups.EditorLinkBar),
    },
    "editor-toolbar": {
        tags: ["div"],
        many: ["blockLabel", "blockOption", "command"],
        load: () => import("../src/components/editor/toolbar/index").then((module) => module.editorToolbar satisfies setups.EditorToolbar),
    },
    "menu": {
        tags: ["div"],
        many: [],
        load: () => import("../src/components/menu/index").then((module) => module.menu satisfies setups.Menu),
    },
    "music-editor": {
        tags: ["figure"],
        many: [],
        load: () => import("../src/blocks/music/index").then((module) => module.musicEditor satisfies setups.MusicEditor),
    },
    "newsletter-editor": {
        tags: ["div"],
        many: [],
        load: () => import("../src/components/editor/index").then((module) => module.newsletterEditor satisfies setups.NewsletterEditor),
    },
    "password-toggle": {
        tags: ["div"],
        many: [],
        load: () => import("./boutique/input/index").then((module) => module.passwordToggle satisfies setups.PasswordToggle),
    },
    "theme-preview": {
        tags: ["form"],
        many: [],
        load: () => import("../src/components/theme_form/index").then((module) => module.themePreview satisfies setups.ThemePreview),
    },
};
