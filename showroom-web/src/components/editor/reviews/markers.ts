import { type Change, ChangeSet, simplifyChanges } from "prosemirror-changeset"
import { DOMSerializer, type Node } from "prosemirror-model"
import { type EditorState, Plugin, PluginKey } from "prosemirror-state"
import { StepMap } from "prosemirror-transform"
import { Decoration, DecorationSet, type EditorView } from "prosemirror-view"
import { schema } from "../prosemirror/schema"

interface Block {
    node: Node
    pos: number
}

interface Hunk {
    base: [number, number]
    next: [number, number]
}

type Kind = "added" | "changed" | "removed"

const markerKey = new PluginKey<ReadonlySet<string>>("review-markers")

const isEmptyParagraph = (node: Node) => node.type === schema.nodes.paragraph && node.content.size === 0

const topLevel = (doc: Node): Array<Block> => {
    const blocks: Array<Block> = []
    doc.forEach((node, pos) => blocks.push({ node, pos }))
    while (blocks.length > 0 && isEmptyParagraph(blocks[blocks.length - 1].node)) {
        blocks.pop()
    }
    return blocks
}

const posAt = (blocks: ReadonlyArray<Block>, index: number): number => {
    if (index < blocks.length) {
        return blocks[index].pos
    }
    const last = blocks.at(-1)
    return last ? last.pos + last.node.nodeSize : 0
}

const lcsHunks = (a: ReadonlyArray<Node>, b: ReadonlyArray<Node>): Array<Hunk> => {
    const lengths = Array.from({ length: a.length + 1 }, () => new Array<number>(b.length + 1).fill(0))
    for (let i = a.length - 1; i >= 0; i--) {
        for (let j = b.length - 1; j >= 0; j--) {
            lengths[i][j] = a[i].eq(b[j]) ? lengths[i + 1][j + 1] + 1 : Math.max(lengths[i + 1][j], lengths[i][j + 1])
        }
    }

    const hunks: Array<Hunk> = []
    let i = 0
    let j = 0
    let startI = 0
    let startJ = 0
    while (i < a.length || j < b.length) {
        if (i < a.length && j < b.length && a[i].eq(b[j])) {
            if (startI < i || startJ < j) {
                hunks.push({ base: [startI, i], next: [startJ, j] })
            }
            i++
            j++
            startI = i
            startJ = j
        } else if (j < b.length && (i === a.length || lengths[i][j + 1] >= lengths[i + 1][j])) {
            j++
        } else {
            i++
        }
    }
    if (startI < a.length || startJ < b.length) {
        hunks.push({ base: [startI, a.length], next: [startJ, b.length] })
    }
    return hunks
}

const diffBlocks = (a: ReadonlyArray<Node>, b: ReadonlyArray<Node>): Array<Hunk> => {
    let start = 0
    while (start < a.length && start < b.length && a[start].eq(b[start])) {
        start++
    }
    let endA = a.length
    let endB = b.length
    while (endA > start && endB > start && a[endA - 1].eq(b[endB - 1])) {
        endA--
        endB--
    }
    return lcsHunks(a.slice(start, endA), b.slice(start, endB)).map((hunk) => ({
        base: [hunk.base[0] + start, hunk.base[1] + start],
        next: [hunk.next[0] + start, hunk.next[1] + start],
    }))
}

const kindOf = (hunk: Hunk): Kind => {
    if (hunk.base[0] === hunk.base[1]) {
        return "added"
    }
    return hunk.next[0] === hunk.next[1] ? "removed" : "changed"
}

const toggle = (view: EditorView, key: string) => {
    const expanded = new Set(markerKey.getState(view.state))
    if (!expanded.delete(key)) {
        expanded.add(key)
    }
    view.dispatch(view.state.tr.setMeta(markerKey, expanded))
}

const textStart = (node: Node, pos: number): number | null => {
    if (node.isTextblock) {
        return pos + 1
    }
    let found: number | null = null
    node.descendants((child, offset) => {
        if (found === null && child.isTextblock) {
            found = pos + 1 + offset + 1
        }
        return found === null
    })
    return found
}

const toggleButton = (view: EditorView, key: string, kind: Kind, isOpen: boolean): HTMLButtonElement => {
    const button = document.createElement("button")
    button.type = "button"
    button.className = `diff-toggle is-${kind}`
    button.contentEditable = "false"
    button.setAttribute("aria-expanded", String(isOpen))
    button.setAttribute("aria-label", isOpen ? "Hide changes" : "Show changes")
    button.title = isOpen ? "Hide changes" : "Show changes"
    button.addEventListener("mousedown", (event) => event.preventDefault())
    button.addEventListener("click", () => toggle(view, key))
    return button
}

const isBlank = (dom: globalThis.Node): boolean =>
    dom.textContent?.trim() === "" && !(dom instanceof Element && dom.matches("img, hr, iframe, :has(img, hr, iframe)"))

const deletedBlocks = (base: Node, from: number, to: number): HTMLElement => {
    const del = document.createElement("del")
    del.className = "diff-block"
    const serializer = DOMSerializer.fromSchema(schema)
    base.slice(from, to).content.forEach((node) => {
        const declared = node.type.spec.diffLabel as string | undefined
        const dom = declared === undefined ? serializer.serializeNode(node) : null
        if (dom && !isBlank(dom)) {
            del.append(dom)
            return
        }
        const paragraph = document.createElement("p")
        paragraph.className = "diff-media-label"
        paragraph.textContent = declared ?? node.type.name
        del.append(paragraph)
    })
    return del
}

const deletedText = (text: string) => (): HTMLElement => {
    const del = document.createElement("del")
    del.textContent = text
    return del
}

const widget = { ignoreSelection: true, stopEvent: () => true }

const hunkDecorations = (state: EditorState, base: Node, hunkChanges: ReadonlyArray<Change>): Array<Decoration> => {
    const decorations: Array<Decoration> = []
    for (const change of hunkChanges) {
        if (change.toB > change.fromB) {
            decorations.push(Decoration.inline(change.fromB, change.toB, { nodeName: "ins" }))
        }
        if (change.toA > change.fromA) {
            const isTopLevel = state.doc.resolve(change.fromB).depth === 0
            const slice = base.slice(change.fromA, change.toA)
            if (isTopLevel && slice.openStart === 0 && slice.openEnd === 0) {
                decorations.push(Decoration.widget(change.fromB, () => deletedBlocks(base, change.fromA, change.toA), { ...widget, side: -1, key: `del-${change.fromA}-${change.toA}` }))
            } else {
                const text = base.textBetween(change.fromA, change.toA, " ", " ")
                if (text.trim() !== "") {
                    decorations.push(Decoration.widget(change.fromB, deletedText(text), { ...widget, side: -1, marks: [], key: `del-${change.fromA}-${change.toA}` }))
                }
            }
        }
    }
    return decorations
}

const decorate = (state: EditorState, base: Node, baseBlocks: ReadonlyArray<Block>): DecorationSet => {
    const expanded = markerKey.getState(state) ?? new Set<string>()
    const blocks = topLevel(state.doc)
    const hunks = diffBlocks(baseBlocks.map(({ node }) => node), blocks.map(({ node }) => node))
    if (hunks.length === 0) {
        return DecorationSet.empty
    }

    const ranges: Array<number> = []
    for (const hunk of hunks) {
        const fromA = posAt(baseBlocks, hunk.base[0])
        ranges.push(fromA, posAt(baseBlocks, hunk.base[1]) - fromA, posAt(blocks, hunk.next[1]) - posAt(blocks, hunk.next[0]))
    }
    const changes = simplifyChanges(ChangeSet.create(base).addSteps(state.doc, [new StepMap(ranges)], null).changes, state.doc)

    const decorations: Array<Decoration> = []
    for (const hunk of hunks) {
        const kind = kindOf(hunk)
        const key = `${hunk.base[0]}-${hunk.base[1]}`
        const isOpen = expanded.has(key)
        const fromB = posAt(blocks, hunk.next[0])
        const toB = posAt(blocks, hunk.next[1])

        if (kind === "removed") {
            decorations.push(Decoration.widget(fromB, (view) => {
                const box = document.createElement("div")
                box.className = "diff-removed"
                box.contentEditable = "false"
                box.append(toggleButton(view, key, kind, isOpen))
                if (isOpen) {
                    box.append(deletedBlocks(base, posAt(baseBlocks, hunk.base[0]), posAt(baseBlocks, hunk.base[1])))
                }
                return box
            }, { ...widget, side: -1, key: `removed-${key}-${isOpen}` }))
            continue
        }

        for (let index = hunk.next[0]; index < hunk.next[1]; index++) {
            const { node, pos } = blocks[index]
            const start = textStart(node, pos)
            const media = start === null ? ` is-media is-${kind}` : ""
            decorations.push(Decoration.node(pos, pos + node.nodeSize, { class: `diff-block${media}` }))
            if (start !== null) {
                decorations.push(Decoration.widget(start, (view) => toggleButton(view, key, kind, isOpen), { ...widget, side: -1, marks: [], key: `toggle-${key}-${index}-${isOpen}` }))
            }
        }

        if (isOpen) {
            decorations.push(...hunkDecorations(state, base, changes.filter((change) => change.fromB >= fromB && change.fromB <= toB)))
        }
    }

    return DecorationSet.create(state.doc, decorations)
}

export const markersPlugin = (base: Node) => {
    const baseBlocks = topLevel(base)
    let cached: { doc: Node, expanded: ReadonlySet<string> | undefined, set: DecorationSet } | undefined

    return new Plugin<ReadonlySet<string>>({
        key: markerKey,
        state: {
            init: () => new Set(),
            apply: (transaction, value) => (transaction.getMeta(markerKey) as ReadonlySet<string> | undefined) ?? value,
        },
        props: {
            decorations: (state) => {
                const expanded = markerKey.getState(state)
                if (cached?.doc !== state.doc || cached.expanded !== expanded) {
                    cached = { doc: state.doc, expanded, set: decorate(state, base, baseBlocks) }
                }
                return cached.set
            },
        },
    })
}
