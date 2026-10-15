import { InputRule, inputRules, textblockTypeInputRule, wrappingInputRule } from "prosemirror-inputrules";
import { TextSelection } from "prosemirror-state";
import { schema } from "./schema";

export function buildInputRules() {
    const { heading, blockquote, bulletList, orderedList, codeBlock, horizontalRule, paragraph } = schema.nodes;

    const divider = new InputRule(/^(?:---|___|\*\*\*)$/, (state, _match, start, end) => {
        const $start = state.doc.resolve(start);
        if ($start.depth < 1 || $start.parent.type !== paragraph || $start.parent.content.size !== end - start) return null;

        const before = $start.before();
        const tr = state.tr.replaceWith(before, $start.after(), [horizontalRule.create(), paragraph.create()]);
        return tr.setSelection(TextSelection.create(tr.doc, before + 2));
    });

    return inputRules({
        rules: [
            textblockTypeInputRule(/^(#{1,4})\s$/, heading, (match) => ({ level: Math.max(2, match[1].length) })),
            wrappingInputRule(/^\s*>\s$/, blockquote),
            wrappingInputRule(/^\s*([-+*])\s$/, bulletList),
            wrappingInputRule(
                /^(\d+)\.\s$/,
                orderedList,
                (match) => ({ start: Number(match[1]) }),
                (match, node) => node.childCount + node.attrs.start === Number(match[1]),
            ),
            textblockTypeInputRule(/^```$/, codeBlock),
            divider,
        ],
    });
}
