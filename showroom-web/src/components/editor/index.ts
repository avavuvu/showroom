import { createApp } from "vue";
import type { NewsletterEditor } from "@setups";
import Editor from "./Editor.vue";

export const newsletterEditor: NewsletterEditor = (element, { signal }) => {
    const props = JSON.parse(element.dataset.props ?? "{}") as Record<string, unknown>;
    const app = createApp(Editor, props);
    app.mount(element);
    signal.addEventListener("abort", () => app.unmount());
};
