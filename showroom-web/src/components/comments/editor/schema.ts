import { Schema } from "prosemirror-model"
import {
    blockquoteSpec,
    boldSpec,
    bulletListSpec,
    codeSpec,
    hardBreakSpec,
    italicSpec,
    linkSpec,
    listItemSpec,
    orderedListSpec,
    paragraphSpec,
    textSpec,
} from "../../editor/prosemirror/specs"

export const commentSchema = new Schema({
    nodes: {
        doc: { content: "block+" },
        paragraph: paragraphSpec,
        blockquote: blockquoteSpec,
        bulletList: bulletListSpec,
        orderedList: orderedListSpec,
        listItem: listItemSpec,
        text: textSpec,
        hardBreak: hardBreakSpec,
    },
    marks: {
        link: linkSpec,
        bold: boldSpec,
        italic: italicSpec,
        code: codeSpec,
    },
})
