import type { Registry } from "@bq/components/src/setups";
import type * as setups from "./setups";

export const registry: Registry = {
    "newsletter-editor": {
        tags: ["sr-editor"],
        many: [],
        load: () => import("../src/views/components/editor/index").then((module) => module.newsletterEditor satisfies setups.NewsletterEditor),
    },
    "password-toggle": {
        tags: ["div"],
        many: [],
        load: () => import("@bq/components/src/input/index").then((module) => module.passwordToggle satisfies setups.PasswordToggle),
    },
    "save-status": {
        tags: ["sr-save-status"],
        many: [],
        load: () => import("../src/views/components/save_status/index").then((module) => module.saveStatus satisfies setups.SaveStatus),
    },
};
