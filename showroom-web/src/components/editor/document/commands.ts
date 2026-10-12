import { setBlockType, wrapIn } from "prosemirror-commands";
import { Fragment, type MarkType, type Node, type NodeType } from "prosemirror-model";
import { liftListItem, wrapInList } from "prosemirror-schema-list";
import { type Command, type EditorState, NodeSelection, Selection, TextSelection } from "prosemirror-state";
import { liftTarget } from "prosemirror-transform";
import { schema } from "./schema";

interface Ancestor {
    node: Node;
    pos: number;
    depth: number;
}

const {
    paragraph,
    heading,
    codeBlock,
    blockquote,
    pullquote,
    attribution,
    bulletList,
    orderedList,
    listItem,
    horizontalRule,
    hardBreak,
    caption,
} = schema.nodes;

export function findAncestor(state: EditorState, predicate: (node: Node) => boolean): Ancestor | null {
    const { $from } = state.selection;
    for (let depth = $from.depth; depth > 0; depth--) {
        const node = $from.node(depth);
        if (predicate(node)) return { node, pos: $from.before(depth), depth };
    }
    return null;
}

export const isList = (node: Node) => node.type === bulletList || node.type === orderedList;

export function isMarkActive(state: EditorState, type: MarkType): boolean {
    const { from, to, empty, $from } = state.selection;
    if (empty) return !!type.isInSet(state.storedMarks ?? $from.marks());
    return state.doc.rangeHasMark(from, to, type);
}

export function canInsert(state: EditorState, type: NodeType): boolean {
    const { $from } = state.selection;
    for (let depth = $from.depth; depth >= 0; depth--) {
        const index = $from.index(depth);
        if ($from.node(depth).canReplaceWith(index, index, type)) return true;
    }
    return false;
}

export const toggleList =
    (type: NodeType): Command =>
    (state, dispatch) => {
        const list = findAncestor(state, isList);
        if (list && list.node.type === type) return liftListItem(listItem)(state, dispatch);
        if (list) {
            if (dispatch) dispatch(state.tr.setNodeMarkup(list.pos, type, type === orderedList ? { start: 1 } : null));
            return true;
        }
        return wrapInList(type)(state, dispatch);
    };

export const toggleBlockquote: Command = (state, dispatch) => {
    const found = findAncestor(state, (node) => node.type === blockquote);
    if (!found) return wrapIn(blockquote)(state, dispatch);

    const { $from, $to } = state.selection;
    const range = $from.blockRange($to, (node) => node.type === blockquote);
    const target = range && liftTarget(range);
    if (!range || target == null) return false;
    if (dispatch) dispatch(state.tr.lift(range, target).scrollIntoView());
    return true;
};

export const togglePullquote: Command = (state, dispatch) => {
    const found = findAncestor(state, (node) => node.type === pullquote);
    if (found) {
        const blocks: Node[] = [];
        found.node.forEach((child) => {
            if (child.type !== attribution) blocks.push(child);
            else if (child.content.size > 0) blocks.push(paragraph.create(null, child.content));
        });
        if (dispatch) {
            const tr = state.tr.replaceWith(found.pos, found.pos + found.node.nodeSize, blocks);
            dispatch(tr.setSelection(Selection.near(tr.doc.resolve(Math.min(state.selection.from - 1, tr.doc.content.size)))).scrollIntoView());
        }
        return true;
    }

    const quote = findAncestor(state, (node) => node.type === blockquote);
    if (quote) {
        const content = Fragment.fromArray([...childrenOf(quote.node), attribution.create()]);
        if (!pullquote.validContent(content)) return false;
        const $quote = state.doc.resolve(quote.pos);
        if (!$quote.parent.canReplaceWith($quote.index(), $quote.index() + 1, pullquote)) return false;
        if (dispatch) {
            const tr = state.tr.replaceWith(quote.pos, quote.pos + quote.node.nodeSize, pullquote.create(null, content));
            dispatch(tr.setSelection(Selection.near(tr.doc.resolve(Math.min(state.selection.from, tr.doc.content.size)))).scrollIntoView());
        }
        return true;
    }

    const { $from, $to } = state.selection;
    const range = $from.blockRange($to);
    if (!range || !range.parent.canReplaceWith(range.startIndex, range.endIndex, pullquote)) return false;

    const blocks: Node[] = [];
    for (let index = range.startIndex; index < range.endIndex; index++) blocks.push(range.parent.child(index));
    const content = Fragment.fromArray([...blocks, attribution.create()]);
    if (!pullquote.validContent(content)) return false;

    if (dispatch) {
        const tr = state.tr.replaceWith(range.start, range.end, pullquote.create(null, content));
        const { from, to } = state.selection;
        dispatch(tr.setSelection(TextSelection.create(tr.doc, from + 1, to + 1)).scrollIntoView());
    }
    return true;
};

function childrenOf(node: Node): Node[] {
    const children: Node[] = [];
    node.forEach((child) => children.push(child));
    return children;
}

export const insertHorizontalRule: Command = (state, dispatch) => {
    if (!canInsert(state, horizontalRule)) return false;
    if (!dispatch) return true;

    const { $from, empty } = state.selection;
    const tr = state.tr;
    if (empty && $from.depth > 0 && $from.parent.type === paragraph && $from.parent.content.size === 0) {
        const start = $from.before();
        tr.replaceWith(start, $from.after(), [horizontalRule.create(), paragraph.create()]);
        tr.setSelection(TextSelection.create(tr.doc, start + 2));
    } else {
        tr.replaceSelectionWith(horizontalRule.create());
    }
    dispatch(tr.scrollIntoView());
    return true;
};

const blockValue = (node: Node): string => {
    if (node.type === paragraph) return "paragraph";
    if (node.type === heading) return `heading-${node.attrs.level}`;
    if (node.type === codeBlock) return "code-block";
    if (node.type === caption) return "caption";
    if (node.type === attribution) return "attribution";
    return "mixed";
};

export function currentBlockType(state: EditorState): string {
    const { from, to, empty, $from } = state.selection;
    if (state.selection instanceof NodeSelection) return blockValue(state.selection.node);
    if (empty) return blockValue($from.parent);

    const values = new Set<string>();
    state.doc.nodesBetween(from, to, (node) => {
        if (!node.isTextblock) return true;
        values.add(blockValue(node));
        return false;
    });
    return values.size === 1 ? [...values][0] : "mixed";
}

export const BLOCK_TYPES = ["paragraph", "heading-2", "heading-3", "heading-4", "code-block"];

export function blockTypeCommand(value: string): Command | null {
    switch (value) {
        case "paragraph":
            return setBlockType(paragraph);
        case "heading-2":
            return setBlockType(heading, { level: 2 });
        case "heading-3":
            return setBlockType(heading, { level: 3 });
        case "heading-4":
            return setBlockType(heading, { level: 4 });
        case "code-block":
            return setBlockType(codeBlock);
        default:
            return null;
    }
}

export function canChangeBlockType(state: EditorState): boolean {
    return BLOCK_TYPES.some((value) => blockTypeCommand(value)?.(state) ?? false);
}
