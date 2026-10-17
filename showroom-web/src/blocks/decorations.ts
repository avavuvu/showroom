import { type EditorState, Plugin, PluginKey, type Transaction } from "prosemirror-state"
import { Decoration, DecorationSet, type EditorProps } from "prosemirror-view"

export interface DecorationChange {
    remove?: object
    add?: Decoration
}

export const cloneTemplate = (template: HTMLTemplateElement): HTMLElement => {
    const cloned = template.content.firstElementChild?.cloneNode(true)
    if (!(cloned instanceof HTMLElement)) {
        throw new Error("the template is empty")
    }
    return cloned
}

export const createWidget = (id: object, pos: number, render: () => HTMLElement): Decoration =>
    Decoration.widget(pos, render, { id, side: -1 })

export class TrackedDecorations {
    private readonly key: PluginKey<DecorationSet>

    constructor(name: string) {
        this.key = new PluginKey<DecorationSet>(name)
    }

    plugin(props: EditorProps<Plugin<DecorationSet>> = {}): Plugin<DecorationSet> {
        const key = this.key
        return new Plugin<DecorationSet>({
            key,
            state: {
                init: () => DecorationSet.empty,
                apply: (transaction, set) => {
                    let next = set.map(transaction.mapping, transaction.doc)
                    const change = transaction.getMeta(key) as DecorationChange | undefined
                    if (change?.remove) {
                        const id = change.remove
                        next = next.remove(next.find(undefined, undefined, (spec) => spec.id === id))
                    }
                    if (change?.add) {
                        next = next.add(transaction.doc, [change.add])
                    }
                    return next
                },
            },
            props: { ...props, decorations: (state) => key.getState(state) },
        })
    }

    change(transaction: Transaction, change: DecorationChange): Transaction {
        return transaction.setMeta(this.key, change)
    }

    find(state: EditorState, id: object): Decoration | undefined {
        return this.key.getState(state)?.find(undefined, undefined, (spec) => spec.id === id)[0]
    }
}
