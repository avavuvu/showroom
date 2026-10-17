import { inject } from "../../../bindings/boutique/setups";
import type { MusicEditor } from "@setups";
import { type MusicService, SERVICES, SERVICE_LABELS, linkCount, withoutLink } from "./attrs";
import { musicNode } from "./handle";

function linkItem(service: MusicService, url: string, removable: boolean): HTMLLIElement {
    const label = SERVICE_LABELS[service];
    const item = document.createElement("li");
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.target = "_blank";
    anchor.rel = "noopener noreferrer";
    anchor.textContent = label;
    const remove = document.createElement("button");
    remove.type = "button";
    remove.dataset.service = service;
    remove.textContent = "\u00d7";
    remove.title = `Remove the ${label} link`;
    remove.setAttribute("aria-label", remove.title);
    remove.hidden = !removable;
    item.append(anchor, remove);
    return item;
}

export const musicEditor: MusicEditor = async (root, { artwork, kind, title, artist, links, add, status }, { signal }) => {
    const node = await inject(root, musicNode);

    const render = () => {
        const attrs = node.attrs();
        const count = linkCount(attrs);

        root.dataset.kind = attrs.kind;
        root.classList.toggle("has-artwork", !!attrs.artwork);
        root.classList.toggle("is-busy", node.busy());
        kind.hidden = attrs.kind !== "album";

        if (attrs.artwork) {
            if (artwork.getAttribute("src") !== attrs.artwork) artwork.src = attrs.artwork;
            artwork.hidden = false;
        } else {
            artwork.removeAttribute("src");
            artwork.hidden = true;
        }

        if (document.activeElement !== title) title.value = attrs.title;
        if (document.activeElement !== artist) artist.value = attrs.artist;

        links.replaceChildren(
            ...SERVICES.flatMap((service) => {
                const link = attrs.links[service];
                return link ? [linkItem(service, link.url, count > 1)] : [];
            }),
        );

        add.disabled = node.busy();
        add.hidden = count >= SERVICES.length;
        status.textContent = node.status();
    };

    node.onUpdate(render, signal);

    return {
        editTitle: () => node.set({ ...node.attrs(), title: title.value }),

        editArtist: () => node.set({ ...node.attrs(), artist: artist.value }),

        async addLink() {
            if (await node.addLink(add.value)) add.value = "";
        },

        clearAdd() {
            add.value = "";
            node.focusEditor();
        },

        linksClick(event) {
            const service = (event.target as Element | null)?.closest<HTMLButtonElement>("button[data-service]")?.dataset.service;
            if (service && linkCount(node.attrs()) > 1) node.set(withoutLink(node.attrs(), service as MusicService));
        },

        pickCover: () => node.pickCover(),

        remove: () => node.remove(),
    };
};
