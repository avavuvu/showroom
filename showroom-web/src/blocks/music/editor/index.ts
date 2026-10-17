import { inject } from "../../../../bindings/boutique/setups"
import type { MusicEditor } from "@setups"
import { setFieldValue, setImageSource } from "../../view"
import { type MusicService, SERVICES, countLinks } from "../attrs"
import { musicEditorKey } from "./handle"

const findService = (item: HTMLElement | null): MusicService | undefined => SERVICES.find((service) => service === item?.dataset.service)

export const musicEditor: MusicEditor = async (root, refs, { signal }) => {
    const node = await inject(root, musicEditorKey)

    const render = () => {
        const attrs = node.getAttrs()
        const linkCount = countLinks(attrs)
        const isBusy = node.isBusy()

        root.dataset.kind = attrs.kind
        root.classList.toggle("has-artwork", !!attrs.artwork)
        root.classList.toggle("has-color", !!attrs.color)
        if (attrs.color) {
            root.style.setProperty("--music-surface", attrs.color)
        } else {
            root.style.removeProperty("--music-surface")
        }
        root.classList.toggle("is-busy", isBusy)
        refs.kind.hidden = attrs.kind !== "album"
        setImageSource(refs.artwork, attrs.artwork)
        setFieldValue(refs.title, attrs.title)
        setFieldValue(refs.artist, attrs.artist)

        for (const item of refs.link) {
            const service = findService(item)
            const link = service && attrs.links[service]
            item.hidden = !link
            const anchor = item.querySelector("a")
            if (anchor && link) {
                anchor.href = link.url
            }
            const button = item.querySelector("button")
            if (button) {
                button.hidden = linkCount < 2
            }
        }

        refs.add.readOnly = isBusy
        refs.add.setAttribute("aria-busy", String(isBusy))
        refs.add.hidden = linkCount >= SERVICES.length
        refs.status.textContent = node.getStatus()
    }

    node.onUpdate(render, signal)

    return {
        editTitle: () => node.change({ title: refs.title.value }),

        editArtist: () => node.change({ artist: refs.artist.value }),

        addLink: async () => {
            if (await node.addLink(refs.add.value)) {
                refs.add.value = ""
            }
        },

        clearAdd: () => {
            refs.add.value = ""
            node.focusEditor()
        },

        unlink: (_event, button) => {
            const service = findService(button.closest("li"))
            if (service) {
                node.unlink(service)
            }
        },

        pickCover: () => node.pickCover(),

        remove: () => node.remove(),
    }
}
