import { type EditorState, Plugin, PluginKey, TextSelection } from "prosemirror-state";
import { Decoration, DecorationSet, type EditorView } from "prosemirror-view";
import { schema } from "../../components/editor/prosemirror/schema";
import { isAllowedHref } from "../../components/editor/prosemirror/url";
import { type Resolved, attrsFrom, hasDetails, looksLikeMusicLink, lookupProblem, resolveMusic } from "./attrs";
import { MusicView } from "./view";

interface LookupMeta {
    add?: { id: object; pos: number; message: string; error: boolean };
    remove?: { id: object };
}

const lookupKey = new PluginKey<DecorationSet>("music-lookup");
const NOTICE_MS = 8_000;

function notice(message: string, error: boolean) {
    return () => {
        const element = document.createElement("figure");
        element.className = error ? "music-card music-lookup is-error" : "music-card music-lookup";
        element.contentEditable = "false";
        const label = document.createElement("span");
        label.setAttribute("role", error ? "alert" : "status");
        label.textContent = message;
        element.append(label);
        return element;
    };
}

function findLookup(state: EditorState, id: object): number | null {
    const found = lookupKey.getState(state)?.find(undefined, undefined, (spec) => spec.id === id) ?? [];
    return found.length > 0 ? found[0].from : null;
}

function isEmptyTopLevelParagraph(state: EditorState): boolean {
    const { selection } = state;
    const { $from } = selection;
    return selection.empty && $from.depth === 1 && $from.parent.type === schema.nodes.paragraph && $from.parent.content.size === 0;
}

export function musicPlugin() {
    return new Plugin<DecorationSet>({
        key: lookupKey,
        state: {
            init: () => DecorationSet.empty,
            apply(tr, set) {
                let next = set.map(tr.mapping, tr.doc);
                const meta = tr.getMeta(lookupKey) as LookupMeta | undefined;
                if (meta?.remove) {
                    const id = meta.remove.id;
                    next = next.remove(next.find(undefined, undefined, (spec) => spec.id === id));
                }
                if (meta?.add) {
                    const { id, pos, message, error } = meta.add;
                    next = next.add(tr.doc, [Decoration.widget(pos, notice(message, error), { id, side: -1 })]);
                }
                return next;
            },
        },
        props: {
            decorations: (state) => lookupKey.getState(state),
            handlePaste(view, event) {
                const text = event.clipboardData?.getData("text/plain")?.trim() ?? "";
                if (!text || /\s/.test(text) || !looksLikeMusicLink(text) || !isEmptyTopLevelParagraph(view.state)) return false;

                const id = {};
                const pos = view.state.selection.$from.before(1);
                view.dispatch(view.state.tr.setMeta(lookupKey, { add: { id, pos, message: "Looking up the link\u2026", error: false } } satisfies LookupMeta));
                void lookup(view, text, id);
                return true;
            },
        },
    });
}

async function lookup(view: EditorView, text: string, id: object) {
    let resolved: Resolved | null = null;
    let failure = "That link could not be looked up.";
    try {
        resolved = await resolveMusic(text);
    } catch (error) {
        if (error instanceof Error) failure = error.message;
    }
    if (view.isDestroyed) return;

    const at = findLookup(view.state, id);
    const tr = view.state.tr.setMeta(lookupKey, { remove: { id } } satisfies LookupMeta);
    if (at == null) {
        view.dispatch(tr);
        return;
    }

    const existing = tr.doc.nodeAt(at);
    const replacing = existing?.type === schema.nodes.paragraph && existing.content.size === 0 ? existing : null;

    if (resolved) {
        const attrs = attrsFrom(resolved);
        const music = schema.nodes.music.create(attrs);
        if (replacing) tr.replaceWith(at, at + replacing.nodeSize, music);
        else tr.insert(at, music);
        view.dispatch(tr.scrollIntoView());

        const card = MusicView.at(view, at);
        const problem = lookupProblem(resolved, null);
        if (problem) card?.announce(problem);
        if (!hasDetails(attrs)) card?.focusTitle();
        return;
    }

    const marks = isAllowedHref(text) ? [schema.marks.link.create({ href: text })] : [];
    const content = schema.text(text, marks);
    if (replacing) {
        tr.insert(at + 1, content);
        tr.setSelection(TextSelection.create(tr.doc, at + 1 + text.length));
    } else {
        tr.insert(at, schema.nodes.paragraph.create(null, content));
    }

    const noticeId = {};
    const message = `No music card was made. ${failure} The link was pasted as text instead.`;
    tr.setMeta(lookupKey, { remove: { id }, add: { id: noticeId, pos: at, message, error: true } } satisfies LookupMeta);
    view.dispatch(tr.scrollIntoView());
    setTimeout(() => {
        if (!view.isDestroyed) view.dispatch(view.state.tr.setMeta(lookupKey, { remove: { id: noticeId } } satisfies LookupMeta));
    }, NOTICE_MS);
}
