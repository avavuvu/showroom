use crate::document::{Block, Document, Inline, InlinePiece, ListItem, Mark, code_text, figure_parts, inline_tree, quote_parts};

pub fn render(document: &Document) -> String {
    blocks(document.blocks())
}

fn blocks(blocks: &[Block]) -> String {
    blocks.iter().map(block).filter(|text| !text.is_empty()).collect::<Vec<_>>().join("\n\n")
}

fn block(block: &Block) -> String {
    match block {
        Block::Paragraph { content }
        | Block::Heading { content, .. }
        | Block::Caption { content }
        | Block::Attribution { content } => inlines(content),
        Block::Blockquote { content, .. } => content_lines(&blocks(content), "> ", ">"),
        Block::Pullquote { content } => {
            let (body, attribution) = quote_parts(content);
            let mut text = blocks(body);
            if !attribution.is_empty() {
                text.push_str("\n\n\u{2014} ");
                text.push_str(&inlines(attribution));
            }
            content_lines(&text, "> ", ">")
        }
        Block::BulletList { content } => content.iter().map(|item| list_item("-", item)).collect::<Vec<_>>().join("\n"),
        Block::OrderedList { attrs, content } => content
            .iter()
            .enumerate()
            .map(|(index, item)| list_item(&format!("{}.", attrs.start as usize + index), item))
            .collect::<Vec<_>>()
            .join("\n"),
        Block::CodeBlock { content, .. } => code_text(content),
        Block::HorizontalRule {} => "---".to_string(),
        Block::Image { attrs } => attrs.src.clone(),
        Block::Figure { content, .. } => match figure_parts(content) {
            Some((image, caption)) if !caption.is_empty() => format!("{} ({})", inlines(caption), image.src),
            Some((image, _)) => image.src.clone(),
            None => String::new(),
        },
    }
}

fn content_lines(text: &str, prefix: &str, empty: &str) -> String {
    text.lines()
        .map(|line| if line.is_empty() { empty.to_string() } else { format!("{prefix}{line}") })
        .collect::<Vec<_>>()
        .join("\n")
}

fn list_item(marker: &str, item: &ListItem) -> String {
    let body = blocks(&item.content);
    let indent = " ".repeat(marker.len() + 1);
    let mut lines = body.lines();
    let mut output = format!("{marker} {}", lines.next().unwrap_or(""));
    for line in lines {
        output.push('\n');
        if !line.is_empty() {
            output.push_str(&indent);
            output.push_str(line);
        }
    }
    output.trim_end().to_string()
}

fn inlines(content: &[Inline]) -> String {
    pieces(&inline_tree(content))
}

fn pieces(pieces: &[InlinePiece]) -> String {
    pieces
        .iter()
        .map(|piece| match piece {
            InlinePiece::Text(text) => text.to_string(),
            InlinePiece::Break => "\n".to_string(),
            InlinePiece::Marked(Mark::Link { attrs }, children) => {
                let text = self::pieces(children);
                let shown = attrs.href.strip_prefix("mailto:").or_else(|| attrs.href.strip_prefix("tel:")).unwrap_or(&attrs.href);
                if text.trim() == shown || text.trim() == attrs.href {
                    text
                } else {
                    format!("{text} ({shown})")
                }
            }
            InlinePiece::Marked(_, children) => self::pieces(children),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::render;
    use crate::document::Document;

    fn text(content: serde_json::Value) -> String {
        render(&Document::from_value(&json!({ "type": "doc", "content": content })).expect("valid document"))
    }

    fn paragraph(value: &str) -> serde_json::Value {
        json!({ "type": "paragraph", "content": [{ "type": "text", "text": value }] })
    }

    fn item(value: &str) -> serde_json::Value {
        json!({ "type": "listItem", "content": [paragraph(value)] })
    }

    #[test]
    fn plain_blocks() {
        let output = text(json!([
            { "type": "heading", "attrs": { "level": 2 }, "content": [{ "type": "text", "text": "Title" }] },
            paragraph("<b>not html</b> & *not markdown*"),
            { "type": "paragraph" },
            { "type": "horizontalRule" },
            { "type": "codeBlock", "content": [{ "type": "text", "text": "a\n  b" }] }
        ]));
        assert_eq!(output, "Title\n\n<b>not html</b> & *not markdown*\n\n---\n\na\n  b");
    }

    #[test]
    fn marks_are_removed_but_link_targets_are_kept() {
        let output = text(json!([
            { "type": "paragraph", "content": [
                { "type": "text", "text": "Read ", "marks": [{ "type": "bold" }] },
                { "type": "text", "text": "the post", "marks": [{ "type": "link", "attrs": { "href": "https://example.com/post" } }] },
                { "type": "text", "text": " or mail " },
                { "type": "text", "text": "ava@example.com", "marks": [{ "type": "link", "attrs": { "href": "mailto:ava@example.com" } }] },
                { "type": "text", "text": " or " },
                { "type": "text", "text": "https://example.com", "marks": [{ "type": "link", "attrs": { "href": "https://example.com" } }] }
            ] }
        ]));
        assert_eq!(output, "Read the post (https://example.com/post) or mail ava@example.com or https://example.com");
    }

    #[test]
    fn hard_breaks_are_new_lines() {
        let output = text(json!([
            { "type": "paragraph", "content": [{ "type": "text", "text": "a" }, { "type": "hardBreak" }, { "type": "text", "text": "b" }] }
        ]));
        assert_eq!(output, "a\nb");
    }

    #[test]
    fn lists_are_numbered_and_nested() {
        let output = text(json!([
            { "type": "orderedList", "content": [
                item("one"),
                { "type": "listItem", "content": [paragraph("two"), { "type": "bulletList", "content": [item("inner")] }] }
            ] },
            { "type": "bulletList", "content": [item("dot")] }
        ]));
        assert_eq!(output, "1. one\n2. two\n\n   - inner\n\n- dot");
    }

    #[test]
    fn attributed_quotes() {
        let output = text(json!([
            { "type": "pullquote", "content": [paragraph("a"), { "type": "attribution", "content": [{ "type": "text", "text": "Ava" }] }] }
        ]));
        assert_eq!(output, "> a\n>\n> \u{2014} Ava");
    }

    #[test]
    fn quotes_and_images() {
        let output = text(json!([
            { "type": "blockquote", "content": [paragraph("a"), paragraph("b")] },
            { "type": "image", "attrs": { "src": "https://res.cloudinary.com/demo/a.jpg", "caption": "A cat" } },
            { "type": "image", "attrs": { "src": "https://res.cloudinary.com/demo/b.jpg", "caption": "", "alt": "IMG_1.jpg" } }
        ]));
        assert_eq!(output, "> a\n>\n> b\n\nA cat (https://res.cloudinary.com/demo/a.jpg)\n\nhttps://res.cloudinary.com/demo/b.jpg");
    }
}
