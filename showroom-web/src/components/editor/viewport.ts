import type { EditorView } from "prosemirror-view";

const MOBILE = "(max-width: 48rem)";
const GAP = 16;

export function visibleBounds(): { top: number; bottom: number } {
    const viewport = window.visualViewport;
    const top = viewport?.offsetTop ?? 0;
    return { top, bottom: top + (viewport?.height ?? window.innerHeight) };
}

export function trackViewport(root: HTMLElement, header: HTMLElement | null, toolbar: HTMLElement, view: EditorView, signal: AbortSignal) {
    const mobile = window.matchMedia(MOBILE);
    let frame = 0;

    const update = () => {
        frame = 0;
        const viewport = window.visualViewport;
        const inset = viewport ? Math.max(0, Math.round(window.innerHeight - viewport.height - viewport.offsetTop)) : 0;
        const toolbarHeight = Math.ceil(toolbar.getBoundingClientRect().height);
        const headerHeight = Math.ceil(header?.getBoundingClientRect().height ?? 0);

        root.style.setProperty("--keyboard-inset", `${inset}px`);
        root.style.setProperty("--keyboard-closed", inset > 0 ? "0" : "1");
        root.style.setProperty("--editor-toolbar-height", `${toolbarHeight}px`);

        view.setProps({
            scrollMargin: mobile.matches
                ? { top: headerHeight + GAP, bottom: toolbarHeight + GAP, left: 0, right: 0 }
                : { top: headerHeight + toolbarHeight + GAP, bottom: GAP, left: 0, right: 0 },
        });
    };

    const schedule = () => {
        if (!frame) frame = requestAnimationFrame(update);
    };

    window.visualViewport?.addEventListener("resize", schedule, { signal });
    window.visualViewport?.addEventListener("scroll", schedule, { signal });
    window.addEventListener("resize", schedule, { signal });
    mobile.addEventListener("change", schedule, { signal });

    const observer = new ResizeObserver(schedule);
    observer.observe(toolbar);
    if (header) observer.observe(header);

    signal.addEventListener(
        "abort",
        () => {
            cancelAnimationFrame(frame);
            observer.disconnect();
        },
        { once: true },
    );

    update();
}
