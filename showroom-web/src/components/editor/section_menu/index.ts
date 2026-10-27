import { inject } from "../../../../bindings/boutique/setups"
import type { EditorSectionMenu } from "@setups"
import { TextSelection } from "prosemirror-state"
import type { EditorView } from "prosemirror-view"
import { editorContext } from "../context"
import { schema } from "../prosemirror/schema"
import { type Section, sectionAt } from "../prosemirror/sections"

interface Heading {
    element: HTMLElement
    pos: number
}

interface Drag {
    pointerId: number
    startY: number
    isMoving: boolean
    target: number | null
}

const HOVER_SLOP = 4
const DRAG_THRESHOLD = 4
const SCROLL_EDGE = 60
const SCROLL_STEP = 12

const getMovableRange = (view: EditorView, section: Section): { from: number, to: number } => {
    const { doc } = view.state
    let to = section.to
    if (to === doc.content.size) {
        const blocks: Array<{ pos: number, isEmpty: boolean }> = []
        doc.forEach((node, pos) => {
            if (pos > section.from && pos < section.to) {
                blocks.push({ pos, isEmpty: node.type === schema.nodes.paragraph && node.content.size === 0 })
            }
        })
        while (blocks.length > 0 && blocks[blocks.length - 1].isEmpty) {
            to = blocks.pop()?.pos ?? to
        }
    }
    return { from: section.from, to }
}

export const editorSectionMenu: EditorSectionMenu = async (root, refs, { signal }) => {
    const { button, drop, handle, outline, option: options } = refs
    const { view, onUpdate, reviews } = await inject(root, editorContext)
    const container = root.parentElement ?? root
    const setLength = (name: string, value: number) => root.style.setProperty(name, `${value}px`)
    const panel = root.querySelector<HTMLElement>(".menu-panel")

    let hovered: HTMLElement | null = null
    let isPreviewing = false
    let active: Heading | null = null
    let drag: Drag | null = null
    let shouldSuppressClick = false

    const isOpen = () => panel?.matches(":popover-open") ?? false

    const getHeadings = (): Array<Heading> => {
        const headings: Array<Heading> = []
        view.state.doc.forEach((node, pos) => {
            const element = view.nodeDOM(pos)
            if (node.type === schema.nodes.heading && element instanceof HTMLElement && element.getClientRects().length > 0) {
                headings.push({ element, pos })
            }
        })
        return headings
    }

    const getFocused = (headings: Array<Heading>): Heading | null => {
        if (!view.hasFocus()) {
            return null
        }
        const { $from } = view.state.selection
        if ($from.depth !== 1 || $from.parent.type !== schema.nodes.heading) {
            return null
        }
        const pos = $from.before(1)
        return headings.find((heading) => heading.pos === pos) ?? null
    }

    const getBoundaries = (range: { from: number, to: number }): Array<{ pos: number, y: number }> => {
        const blocks: Array<{ pos: number, rect: DOMRect }> = []
        view.state.doc.forEach((_node, pos) => {
            const element = view.nodeDOM(pos)
            if (element instanceof HTMLElement) {
                blocks.push({ pos, rect: element.getBoundingClientRect() })
            }
        })
        const boundaries = blocks.map(({ pos, rect }, index) => ({
            pos,
            y: index === 0 ? rect.top : (blocks[index - 1].rect.bottom + rect.top) / 2,
        }))
        const last = blocks.at(-1)
        if (last) {
            boundaries.push({ pos: view.state.doc.content.size, y: last.rect.bottom })
        }
        return boundaries.filter(({ pos }) => pos < range.from || pos > range.to)
    }

    const render = () => {
        const headings = getHeadings()
        const current = headings.find((heading) => heading.element === hovered) ?? getFocused(headings)
        active = current
        if (!current) {
            hovered = null
        }

        const base = root.getBoundingClientRect()
        handle.classList.toggle("is-visible", !!current)
        handle.classList.toggle("is-dragging", !!drag?.isMoving)
        if (current) {
            const rect = current.element.getBoundingClientRect()
            handle.dataset.level = current.element.tagName.slice(1)
            setLength("--heading-top", rect.top - base.top)
            setLength("--heading-height", rect.height)
        }

        const section = current && (isPreviewing || isOpen() || drag?.isMoving) ? sectionAt(view.state.doc, current.pos) : null
        outline.classList.toggle("is-visible", !!section)
        if (section && current) {
            const range = getMovableRange(view, section)
            let lastPos = range.from
            view.state.doc.forEach((_node, pos) => {
                if (pos >= range.from && pos < range.to) {
                    lastPos = pos
                }
            })
            const lastElement = view.nodeDOM(lastPos)
            const last = lastElement instanceof HTMLElement ? lastElement : current.element
            setLength("--section-bottom", last.getBoundingClientRect().bottom - base.top)
        }

        const target = drag?.isMoving && section ? getBoundaries(getMovableRange(view, section)).find(({ pos }) => pos === drag?.target) : null
        drop.classList.toggle("is-visible", !!target)
        if (target) {
            setLength("--drop-top", target.y - base.top)
        }
    }

    const pick = (y: number): HTMLElement | null =>
        getHeadings().find(({ element }) => {
            const rect = element.getBoundingClientRect()
            return y >= rect.top - HOVER_SLOP && y <= rect.bottom + HOVER_SLOP
        })?.element ?? null

    container.addEventListener("pointermove", (event) => {
        if (drag || event.pointerType === "touch" || isOpen() || handle.contains(event.target as Node)) {
            return
        }
        const next = pick(event.clientY)
        if (next !== hovered) {
            hovered = next
            render()
        }
    }, { signal })

    container.addEventListener("pointerleave", () => {
        if (!drag && !isOpen() && !isPreviewing) {
            hovered = null
            render()
        }
    }, { signal })

    const setPreviewing = (isOn: boolean) => {
        isPreviewing = isOn
        render()
    }
    button.addEventListener("pointerenter", () => setPreviewing(true), { signal })
    button.addEventListener("pointerleave", () => setPreviewing(false), { signal })
    button.addEventListener("focus", () => setPreviewing(true), { signal })
    button.addEventListener("blur", () => setPreviewing(false), { signal })
    panel?.addEventListener("beforetoggle", () => syncOptions(), { signal })
    panel?.addEventListener("toggle", () => render(), { signal })

    const endDrag = () => {
        if (drag && button.hasPointerCapture(drag.pointerId)) {
            button.releasePointerCapture(drag.pointerId)
        }
        drag = null
        render()
    }

    const moveSection = (target: number) => {
        const section = active && sectionAt(view.state.doc, active.pos)
        if (!section) {
            return
        }
        const { from, to } = getMovableRange(view, section)
        if (target >= from && target <= to) {
            return
        }
        const content = view.state.doc.slice(from, to).content
        const transaction = view.state.tr.delete(from, to)
        const insertAt = transaction.mapping.map(target)
        transaction.insert(insertAt, content)
        transaction.setSelection(TextSelection.near(transaction.doc.resolve(insertAt + 1)))
        view.dispatch(transaction.scrollIntoView())
        hovered = null
        view.focus()
    }

    button.addEventListener("pointerdown", (event) => {
        if (event.button !== 0 || !active) {
            return
        }
        drag = { pointerId: event.pointerId, startY: event.clientY, isMoving: false, target: null }
        button.setPointerCapture(event.pointerId)
    }, { signal })

    button.addEventListener("pointermove", (event) => {
        if (!drag || event.pointerId !== drag.pointerId) {
            return
        }
        if (!drag.isMoving && Math.abs(event.clientY - drag.startY) < DRAG_THRESHOLD) {
            return
        }
        drag.isMoving = true
        event.preventDefault()

        if (event.clientY < SCROLL_EDGE) {
            window.scrollBy(0, -SCROLL_STEP)
        } else if (event.clientY > window.innerHeight - SCROLL_EDGE) {
            window.scrollBy(0, SCROLL_STEP)
        }

        const section = active && sectionAt(view.state.doc, active.pos)
        const boundaries = section ? getBoundaries(getMovableRange(view, section)) : []
        const nearest = boundaries.reduce<{ pos: number, y: number } | null>(
            (best, boundary) => (!best || Math.abs(boundary.y - event.clientY) < Math.abs(best.y - event.clientY) ? boundary : best),
            null,
        )
        drag.target = nearest?.pos ?? null
        render()
    }, { signal })

    button.addEventListener("pointerup", (event) => {
        if (!drag || event.pointerId !== drag.pointerId) {
            return
        }
        const { isMoving, target } = drag
        endDrag()
        if (isMoving) {
            shouldSuppressClick = true
            if (target !== null) {
                moveSection(target)
            }
        }
    }, { signal })

    button.addEventListener("pointercancel", endDrag, { signal })

    button.addEventListener("click", (event) => {
        if (shouldSuppressClick) {
            shouldSuppressClick = false
            event.preventDefault()
            event.stopImmediatePropagation()
        }
    }, { capture: true, signal })

    window.addEventListener("keydown", (event) => {
        if (drag?.isMoving && event.key === "Escape") {
            event.preventDefault()
            shouldSuppressClick = true
            endDrag()
        }
    }, { capture: true, signal })

    const resize = new ResizeObserver(() => render())
    resize.observe(view.dom)
    signal.addEventListener("abort", () => resize.disconnect(), { once: true })

    onUpdate(() => render(), signal)

    const getSection = (): Section | null => (active ? sectionAt(view.state.doc, active.pos) : null)

    const syncOptions = () => {
        const section = getSection()
        const count = section ? (reviews?.summaries.filter((summary) => summary.sectionId === section.id).length ?? 0) : 0
        for (const item of options) {
            switch (item.dataset.action) {
                case "send":
                    item.hidden = !reviews
                    break
                case "reviews": {
                    item.hidden = count === 0
                    const label = item.querySelector(".option-label")
                    if (label) {
                        label.textContent = `See suggestions (${count})`
                    }
                    break
                }
                default:
                    item.hidden = false
            }
        }
    }

    const selectSection = () => {
        const section = getSection()
        if (!section) {
            return
        }
        const { doc } = view.state
        const selection = TextSelection.between(doc.resolve(section.from), doc.resolve(section.to))
        view.dispatch(view.state.tr.setSelection(selection).scrollIntoView())
        view.focus()
    }

    return {
        choose: (_event, item) => {
            const section = getSection()
            switch (item.dataset.action) {
                case "select":
                    selectSection()
                    return
                case "send":
                    if (section && reviews) {
                        reviews.navigate(reviews.sendUrl(section.id))
                    }
                    return
                case "reviews":
                    if (reviews) {
                        reviews.navigate(reviews.listUrl)
                    }
            }
        },
    }
}
