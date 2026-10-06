import type { Registry } from "@bq/components/src/setups";
import type * as setups from "./setups";

export const registry: Registry = {
    "newsletter-editor": {
        tags: ["div"],
        many: [],
        load: () => import("../src/components/editor/index").then((module) => module.newsletterEditor satisfies setups.NewsletterEditor),
    },
    "password-toggle": {
        tags: ["div"],
        many: [],
        load: () => import("@bq/components/src/input/index").then((module) => module.passwordToggle satisfies setups.PasswordToggle),
    },
    "save-status": {
        tags: ["output"],
        many: [],
        load: () => import("../src/components/save_status/index").then((module) => module.saveStatus satisfies setups.SaveStatus),
    },
};
