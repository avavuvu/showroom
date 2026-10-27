import type { Node } from "prosemirror-model"
import { EditorState, Plugin, type Transaction } from "prosemirror-state"
import { Mapping } from "prosemirror-transform"
import { schema } from "./schema"

const { heading } = schema.nodes

const ID_ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
const ID_PATTERN = /^[A-Za-z0-9_-]{1,32}$/

const createSectionId = (): string => {
    const bytes = crypto.getRandomValues(new Uint8Array(12))
    return Array.from(bytes, (byte) => ID_ALPHABET[byte % ID_ALPHABET.length]).join("")
}

const validId = (value: unknown): string | null => (typeof value === "string" && ID_PATTERN.test(value) ? value : null)

const assignIds = (transaction: Transaction, previous: Map<number, string>): Transaction => {
    const headings: Array<{ pos: number, id: string | null }> = []
    transaction.doc.descendants((node, pos) => {
        if (node.type === heading) {
            headings.push({ pos, id: validId(node.attrs.id) })
            return false
        }
        return !node.isTextblock
    })

    const used = new Set<string>()
    const resolved = new Map<number, string>()
    for (const { pos, id } of headings) {
        const old = previous.get(pos)
        if (old && (id === null || id === old) && !used.has(old)) {
            resolved.set(pos, old)
            used.add(old)
        }
    }
    for (const { pos, id } of headings) {
        if (resolved.has(pos)) {
            continue
        }
        let next = id && !used.has(id) ? id : createSectionId()
        while (used.has(next)) {
            next = createSectionId()
        }
        resolved.set(pos, next)
        used.add(next)
    }

    for (const { pos, id } of headings) {
        const next = resolved.get(pos)
        if (next && next !== id) {
            transaction.setNodeAttribute(pos, "id", next)
        }
    }
    return transaction
}

export const withSectionIds = (doc: Node): Node => assignIds(EditorState.create({ doc }).tr, new Map()).doc

export const sectionIdPlugin = () => new Plugin({
    appendTransaction: (transactions, oldState, newState) => {
        if (!transactions.some((transaction) => transaction.docChanged)) {
            return null
        }
        const mapping = new Mapping()
        transactions.forEach((transaction) => mapping.appendMapping(transaction.mapping))

        const previous = new Map<number, string>()
        oldState.doc.descendants((node, pos) => {
            if (node.type === heading) {
                const id = validId(node.attrs.id)
                if (id) {
                    previous.set(mapping.map(pos, 1), id)
                }
                return false
            }
            return !node.isTextblock
        })

        const transaction = assignIds(newState.tr, previous)
        return transaction.docChanged ? transaction.setMeta("addToHistory", false) : null
    },
})

export interface Section {
    id: string
    level: number
    from: number
    to: number
    heading: Node
}

const getSections = (doc: Node): Array<Section> => {
    const tops: Array<{ node: Node, pos: number }> = []
    doc.forEach((node, pos) => tops.push({ node, pos }))

    const sections: Array<Section> = []
    tops.forEach(({ node, pos }, index) => {
        if (node.type !== heading || !node.attrs.id) {
            return
        }
        const level = node.attrs.level as number
        const next = tops.slice(index + 1).find((top) => top.node.type === heading && (top.node.attrs.level as number) <= level)
        sections.push({ id: node.attrs.id as string, level, from: pos, to: next ? next.pos : doc.content.size, heading: node })
    })
    return sections
}

export const sectionAt = (doc: Node, pos: number): Section | null =>
    getSections(doc).find((section) => section.from === pos) ?? null
