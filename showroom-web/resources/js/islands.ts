import { createApp, type Component } from "vue";

declare global {
    function mountIslands(
        modules: Record<string, () => Promise<{ default: Component }>>,
        mount: (element: HTMLElement, component: Component, props: Record<string, unknown>) => void,
    ): void;
}

mountIslands(import.meta.glob<{ default: Component }>("./islands/*.vue"), (element, component, props) => {
    createApp(component, props).mount(element);
});
