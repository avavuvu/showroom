import { toggleMark } from "prosemirror-commands";
import { redo, undo } from "prosemirror-history";
import type { NodeType } from "prosemirror-model";
import type { Command, EditorState } from "prosemirror-state";
import { findAncestor, insertHorizontalRule, isList, isMarkActive, toggleBlockquote, toggleList, togglePullquote } from "../document/commands";
import { schema } from "../document/schema";

export interface ToolbarCommand {
    run: Command;
    active?: (state: EditorState) => boolean;
}

const { blockquote, pullquote, bulletList, orderedList } = schema.nodes;

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
