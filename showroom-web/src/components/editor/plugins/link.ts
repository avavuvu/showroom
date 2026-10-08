import type { Mark, ResolvedPos } from "prosemirror-model";
import { type Command, type EditorState, Plugin, PluginKey, TextSelection } from "prosemirror-state";
import { Decoration, DecorationSet, type EditorView } from "prosemirror-view";
import { schema } from "../schema";
import { displayHref, normalizeHref } from "../url";

export interface Editing {
    from: number;
    to: number;
    href: string;
}

interface LinkState {
    editing: Editing | null;
}

type LinkMeta = { open: Editing } | { close: true };

export interface LinkRange {
    from: number;
    to: number;
    mark: Mark;
}

const linkKey = new PluginKey<LinkState>("link");
const link = schema.marks.link;

function linkRangeAt($pos: ResolvedPos): LinkRange | null {
    const parent = $pos.parent;
    let child = parent.childAfter($pos.parentOffset);
    if (!child.node || !link.isInSet(child.node.marks)) {
        if ($pos.parentOffset === 0) return null;
        child = parent.childBefore($pos.parentOffset);
    }
    const mark = child.node && link.isInSet(child.node.marks);
    if (!child.node || !mark) return null;

    let startIndex = child.index;
    let from = $pos.start() + child.offset;
    let endIndex = startIndex + 1;
    let to = from + child.node.nodeSize;

    while (startIndex > 0 && mark.isInSet(parent.child(startIndex - 1).marks)) {
        startIndex--;
        from -= parent.child(startIndex).nodeSize;
    }
    while (endIndex < parent.childCount && mark.isInSet(parent.child(endIndex).marks)) {
        to += parent.child(endIndex).nodeSize;
        endIndex++;
    }
    return { from, to, mark };
}

export function selectedLink(state: EditorState): LinkRange | null {
    const { selection } = state;
    if (!(selection instanceof TextSelection)) return null;
    const range = linkRangeAt(selection.$from);
    if (!range) return null;
    if (!selection.empty && (selection.from < range.from || selection.to > range.to)) return null;
    return range;
}

export function canLink(state: EditorState): boolean {
    const { selection } = state;
    return (
        selection instanceof TextSelection &&
        selection.$from.parent.type.allowsMarkType(link) &&
        selection.$to.parent.type.allowsMarkType(link)
    );
}

export function linkEditing(state: EditorState): Editing | null {
    return linkKey.getState(state)?.editing ?? null;
}

export const openLink: Command = (state, dispatch) => {
    if (!canLink(state)) return false;
    if (dispatch && !linkEditing(state)) {
        const existing = selectedLink(state);
        const editing: Editing = existing
            ? { from: existing.from, to: existing.to, href: existing.mark.attrs.href }
            : { from: state.selection.from, to: state.selection.to, href: "" };
        dispatch(state.tr.setMeta(linkKey, { open: editing } satisfies LinkMeta));
    }
    return true;
};

export function closeLink(view: EditorView) {
    if (!view.isDestroyed && linkEditing(view.state)) {
        view.dispatch(view.state.tr.setMeta(linkKey, { close: true } satisfies LinkMeta));
    }
}

export function linkText(state: EditorState, range: { from: number; to: number }): string {
    return state.doc.textBetween(range.from, range.to, " ");
}

export function applyLink(view: EditorView, rawHref: string, rawText: string): string | null {
    const { state } = view;
    const editing = linkEditing(state);
    if (!editing) return null;

    const href = rawHref.trim() ? normalizeHref(rawHref) : null;
    if (rawHref.trim() && !href) return "Use a web address, an email address or a phone number for the link.";

    const current = linkText(state, editing);
    const label = rawText.trim() ? rawText : current || (href ? displayHref(href) : "");
    const tr = state.tr;
    let end = editing.to;

    if (label !== current) {
        if (!label) return null;
        const marks = (state.doc.nodeAt(editing.from)?.marks ?? state.selection.$from.marks()).filter((mark) => mark.type !== link);
        tr.replaceWith(editing.from, editing.to, schema.text(label, marks));
        end = editing.from + label.length;
    }

    tr.removeMark(editing.from, end, link);
    if (href) tr.addMark(editing.from, end, link.create({ href }));
    tr.setSelection(TextSelection.create(tr.doc, end));

    view.dispatch(tr.setMeta(linkKey, { close: true } satisfies LinkMeta).scrollIntoView());
    return null;
}

export function removeLink(view: EditorView) {
    const { state } = view;
    const editing = linkEditing(state);
    const range = editing?.href ? editing : selectedLink(state);
    if (!range) return;
    view.dispatch(state.tr.removeMark(range.from, range.to, link).setMeta(linkKey, { close: true } satisfies LinkMeta));
}

export function linkPlugin() {
    return new Plugin<LinkState>({
        key: linkKey,
        state: {
            init: () => ({ editing: null }),
            apply(tr, value) {
                const meta = tr.getMeta(linkKey) as LinkMeta | undefined;
                if (meta && "open" in meta) return { editing: meta.open };
                if (meta && "close" in meta) return { editing: null };
                if (value.editing && tr.docChanged) {
                    const from = tr.mapping.map(value.editing.from, 1);
                    const to = Math.max(from, tr.mapping.map(value.editing.to, -1));
                    return { editing: { ...value.editing, from, to } };
                }
                return value;
            },
        },
        props: {
            decorations(state) {
                const editing = linkEditing(state);
                if (!editing || editing.from === editing.to) return null;
                return DecorationSet.create(state.doc, [Decoration.inline(editing.from, editing.to, { class: "link-target" })]);
            },
            handlePaste(view, event) {
                const { selection } = view.state;
                const text = event.clipboardData?.getData("text/plain")?.trim() ?? "";
                if (selection.empty || !canLink(view.state)) return false;
                if (!/^(https?:\/\/|mailto:)\S+$/i.test(text)) return false;

                const href = normalizeHref(text);
                if (!href) return false;
                view.dispatch(view.state.tr.addMark(selection.from, selection.to, link.create({ href })));
                return true;
            },
        },
    });
}
