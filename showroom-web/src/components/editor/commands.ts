import { chainCommands, exitCode, setBlockType, splitBlockAs, toggleMark, wrapIn } from "prosemirror-commands";
import { redo, undo } from "prosemirror-history";
import { undoInputRule } from "prosemirror-inputrules";
import { Fragment, type MarkType, type Node, NodeRange, type NodeType } from "prosemirror-model";
import { liftListItem, sinkListItem, splitListItem, wrapInList } from "prosemirror-schema-list";
import { type Command, type EditorState, NodeSelection, Selection, TextSelection } from "prosemirror-state";
import { liftTarget } from "prosemirror-transform";
import { schema } from "./schema";

export interface ToolbarCommand {
    run: Command;
    active?: (state: EditorState) => boolean;
}

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

const isList = (node: Node) => node.type === bulletList || node.type === orderedList;

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

const insertHardBreak: Command = (state, dispatch) => {
    if (!canInsert(state, hardBreak)) return false;
    if (dispatch) dispatch(state.tr.replaceSelectionWith(hardBreak.create()).scrollIntoView());
    return true;
};

const inList: Command = (state) => !!findAncestor(state, (node) => node.type === listItem);

const exitCaption: Command = (state, dispatch) => {
    const { $from } = state.selection;
    if ($from.parent.type !== caption && $from.parent.type !== attribution) return false;
    if (!dispatch) return true;

    const after = $from.after($from.depth - 1);
    const next = state.doc.resolve(after).nodeAfter;
    const tr = state.tr;
    if (next?.isTextblock) {
        tr.setSelection(TextSelection.create(tr.doc, after + 1));
    } else {
        tr.insert(after, paragraph.create());
        tr.setSelection(TextSelection.create(tr.doc, after + 1));
    }
    dispatch(tr.scrollIntoView());
    return true;
};

const enterSource: Command = (state, dispatch) => {
    const { $from, empty } = state.selection;
    if (!empty || $from.depth < 2 || $from.parent.type !== paragraph || $from.parent.content.size !== 0) return false;
    const parent = $from.node($from.depth - 1);
    if (parent.type !== pullquote || $from.index($from.depth - 1) !== parent.childCount - 2) return false;
    if (!dispatch) return true;

    const start = $from.before();
    const tr = state.tr;
    if (parent.childCount > 2) tr.delete(start, $from.after());
    dispatch(tr.setSelection(TextSelection.create(tr.doc, parent.childCount > 2 ? start + 1 : $from.after() + 1)).scrollIntoView());
    return true;
};

export const moveBlock =
    (direction: -1 | 1): Command =>
    (state, dispatch) => {
        const { selection } = state;
        let range = selection.$from.blockRange(selection.$to);

        while (range) {
            const { parent, startIndex, endIndex } = range;
            const swapIndex = direction < 0 ? startIndex - 1 : endIndex;

            if (swapIndex >= 0 && swapIndex < parent.childCount) {
                const sibling = parent.child(swapIndex);
                const moved: Node[] = [];
                for (let index = startIndex; index < endIndex; index++) moved.push(parent.child(index));

                const children: Node[] = [];
                parent.forEach((child) => children.push(child));
                const first = Math.min(swapIndex, startIndex);
                const reordered = direction < 0 ? [...moved, sibling] : [sibling, ...moved];
                children.splice(first, reordered.length, ...reordered);

                if (parent.type.validContent(Fragment.fromArray(children))) {
                    if (dispatch) {
                        const shift = direction < 0 ? -sibling.nodeSize : sibling.nodeSize;
                        const from = direction < 0 ? range.start - sibling.nodeSize : range.start;
                        const to = direction < 0 ? range.end : range.end + sibling.nodeSize;
                        const tr = state.tr.replaceWith(from, to, reordered);
                        tr.setSelection(
                            selection instanceof NodeSelection
                                ? NodeSelection.create(tr.doc, selection.from + shift)
                                : selection instanceof TextSelection
                                  ? TextSelection.create(tr.doc, selection.anchor + shift, selection.head + shift)
                                  : Selection.near(tr.doc.resolve(selection.from + shift)),
                        );
                        dispatch(tr.scrollIntoView());
                    }
                    return true;
                }
            }

            if (range.depth === 0) return false;
            const $start = state.doc.resolve(range.$from.before(range.depth));
            const $end = state.doc.resolve(range.$from.after(range.depth));
            range = new NodeRange($start, $end, range.depth - 1);
        }
        return false;
    };

export const insertLine =
    (direction: -1 | 1): Command =>
    (state, dispatch) => {
        const { selection } = state;
        const { $from } = selection;
        if (direction > 0 && $from.parent.type.spec.code) return exitCode(state, dispatch);

        const insert = (pos: number, node: Node, cursor: number) => {
            if (dispatch) {
                const tr = state.tr.insert(pos, node);
                dispatch(tr.setSelection(TextSelection.create(tr.doc, pos + cursor)).scrollIntoView());
            }
            return true;
        };

        if (selection instanceof NodeSelection) {
            const index = direction > 0 ? $from.index() + 1 : $from.index();
            if ($from.parent.canReplaceWith(index, index, paragraph)) {
                return insert(direction > 0 ? selection.to : selection.from, paragraph.create(), 1);
            }
        }

        for (let depth = $from.depth; depth > 0; depth--) {
            const container = $from.node(depth - 1);
            const index = direction > 0 ? $from.indexAfter(depth - 1) : $from.index(depth - 1);
            const pos = direction > 0 ? $from.after(depth) : $from.before(depth);

            if (container.type === listItem && depth > 1) continue;
            if (container.type === bulletList || container.type === orderedList) {
                return insert(pos, listItem.create(null, paragraph.create()), 2);
            }
            if (container.canReplaceWith(index, index, paragraph)) return insert(pos, paragraph.create(), 1);
        }
        return false;
    };

const enterCaption: Command = (state, dispatch) => {
    const { selection } = state;
    if (!(selection instanceof NodeSelection) || selection.node.type !== schema.nodes.figure) return false;
    if (dispatch) {
        const end = selection.from + selection.node.nodeSize - 2;
        dispatch(state.tr.setSelection(TextSelection.create(state.doc, end)).scrollIntoView());
    }
    return true;
};

const splitInPullquote: Command = (state, dispatch) => {
    const { $from } = state.selection;
    if ($from.depth < 2 || $from.node(-1).type !== pullquote || $from.parent.content.size === 0) return false;
    return splitBlockAs((_node, atEnd) => (atEnd ? { type: paragraph } : null))(state, dispatch);
};

const selectFigure: Command = (state, dispatch) => {
    const { $from } = state.selection;
    if ($from.parent.type !== caption) return false;
    if (dispatch) dispatch(state.tr.setSelection(NodeSelection.create(state.doc, $from.before($from.depth - 1))));
    return true;
};

export function buildKeymap(openLink: Command): Record<string, Command> {
    const { bold, italic, underline, strike, code } = schema.marks;
    return {
        "Mod-z": undo,
        "Shift-Mod-z": redo,
        "Mod-y": redo,
        Backspace: undoInputRule,
        "Mod-b": toggleMark(bold),
        "Mod-i": toggleMark(italic),
        "Mod-u": toggleMark(underline),
        "Shift-Mod-x": toggleMark(strike),
        "Mod-e": toggleMark(code),
        "Mod-k": (state, dispatch, view) => {
            openLink(state, dispatch, view);
            return true;
        },

        "Mod-Alt-0": setBlockType(paragraph),
        "Mod-Alt-2": setBlockType(heading, { level: 2 }),
        "Mod-Alt-3": setBlockType(heading, { level: 3 }),
        "Mod-Alt-4": setBlockType(heading, { level: 4 }),
        "Mod-Alt-c": setBlockType(codeBlock),
        "Shift-Mod-7": toggleList(orderedList),
        "Shift-Mod-8": toggleList(bulletList),
        "Shift-Mod-9": toggleBlockquote,
        Enter: chainCommands(enterCaption, exitCaption, enterSource, splitInPullquote, splitListItem(listItem)),
        Escape: selectFigure,
        "Mod-Enter": insertLine(1),
        "Shift-Mod-Enter": insertLine(-1),
        "Alt-ArrowUp": moveBlock(-1),
        "Alt-ArrowDown": moveBlock(1),
        "Shift-Enter": chainCommands(exitCode, insertHardBreak),
        Tab: chainCommands(sinkListItem(listItem), inList),
        "Shift-Tab": chainCommands(liftListItem(listItem), inList),
    };
}

export function toolbarCommands(actions: { openLink: Command; insertImage: Command }): Record<string, ToolbarCommand> {
    const { bold, italic, underline, strike, code, link } = schema.marks;
    const listType = (state: EditorState) => findAncestor(state, isList)?.node.type ?? null;
    const inside = (type: NodeType) => (state: EditorState) => !!findAncestor(state, (node) => node.type === type);

    return {
        undo: { run: undo },
        redo: { run: redo },
        bold: { run: toggleMark(bold), active: (state) => isMarkActive(state, bold) },
        italic: { run: toggleMark(italic), active: (state) => isMarkActive(state, italic) },
        underline: { run: toggleMark(underline), active: (state) => isMarkActive(state, underline) },
        strike: { run: toggleMark(strike), active: (state) => isMarkActive(state, strike) },
        code: { run: toggleMark(code), active: (state) => isMarkActive(state, code) },
        link: { run: actions.openLink, active: (state) => isMarkActive(state, link) },
        "bullet-list": { run: toggleList(bulletList), active: (state) => listType(state) === bulletList },
        "ordered-list": { run: toggleList(orderedList), active: (state) => listType(state) === orderedList },
        blockquote: { run: toggleBlockquote, active: inside(blockquote) },
        pullquote: { run: togglePullquote, active: inside(pullquote) },
        "horizontal-rule": { run: insertHorizontalRule },
        image: { run: actions.insertImage },
    };
}

