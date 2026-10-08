use crate::document::{Block, Document, ImageAttrs, Inline, InlinePiece, ListItem, Mark, code_text, figure_parts, inline_text, inline_tree, quote_parts};

pub fn render(document: &Document) -> String {
    blocks(document.blocks())
}

fn blocks(blocks: &[Block]) -> String {
    blocks.iter().map(block).filter(|text| !text.is_empty()).collect::<Vec<_>>().join("\n\n")
}

fn block(block: &Block) -> String {
    match block {
        Block::Paragraph { content } | Block::Caption { content } | Block::Attribution { content } => escape_line_starts(&inlines(content)),
        Block::Heading { attrs, content } => {
            let text = inlines(content);
            if text.trim().is_empty() {
                return String::new();
            }
            format!("{} {}", "#".repeat(attrs.level.into()), text.replace("\\\n", " "))
        }
        Block::Blockquote { content, .. } => quote(&blocks(content)),
        Block::Pullquote { content } => {
            let (body, attribution) = quote_parts(content);
            let mut text = blocks(body);
            if !attribution.is_empty() {
                text.push_str("\n\n\u{2014} ");
                text.push_str(&inlines(attribution));
            }
            quote(&text)
        }
        Block::BulletList { content } => content.iter().map(|item| list_item("-", item)).collect::<Vec<_>>().join("\n"),
        Block::OrderedList { attrs, content } => content
            .iter()
            .enumerate()
            .map(|(index, item)| list_item(&format!("{}.", attrs.start as usize + index), item))
            .collect::<Vec<_>>()
            .join("\n"),
        Block::CodeBlock { attrs, content } => {
            let text = code_text(content);
            let fence = if text.contains("```") { "~~~~" } else { "```" };
            format!("{fence}{}\n{text}\n{fence}", attrs.language.as_deref().unwrap_or(""))
        }
        Block::HorizontalRule {} => "---".to_string(),
        Block::Image { attrs } => image(attrs, &[]),
        Block::Figure { content, .. } => match figure_parts(content) {
            Some((attrs, caption)) => {
                let mut text = image(attrs, caption);
                if !caption.is_empty() {
                    text.push_str("\n\n");
                    text.push_str(&escape_line_starts(&inlines(caption)));
                }
                text
            }
            None => String::new(),
        },
    }
}

fn image(attrs: &ImageAttrs, caption: &[Inline]) -> String {
    format!("![{}]({})", escape_text(&inline_text(caption)), destination(&attrs.src))
}

fn quote(text: &str) -> String {
    text.lines()
        .map(|line| if line.is_empty() { ">".to_string() } else { format!("> {line}") })
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
            InlinePiece::Text(text) => escape_text(text),
            InlinePiece::Break => "\\\n".to_string(),
            InlinePiece::Marked(mark, children) => marked(mark, children),
        })
        .collect()
}

fn marked(mark: &Mark, children: &[InlinePiece]) -> String {
    match mark {
        Mark::Link { attrs } => format!("[{}]({})", self::pieces(children), destination(&attrs.href)),
        Mark::Bold {} => wrap(&self::pieces(children), "**"),
        Mark::Italic {} => wrap(&self::pieces(children), "*"),
        Mark::Strike {} => wrap(&self::pieces(children), "~~"),
        Mark::Underline {} => self::pieces(children),
        Mark::Code {} => code_span(&raw_text(children)),
    }
}

fn wrap(text: &str, delimiter: &str) -> String {
    let core = text.trim();
    if core.is_empty() {
        return text.to_string();
    }
    let start = text.len() - text.trim_start().len();
    let end = start + core.len();
    format!("{}{delimiter}{core}{delimiter}{}", &text[..start], &text[end..])
}

fn raw_text(pieces: &[InlinePiece]) -> String {
    pieces
        .iter()
        .map(|piece| match piece {
            InlinePiece::Text(text) => text.to_string(),
            InlinePiece::Break => " ".to_string(),
            InlinePiece::Marked(_, children) => raw_text(children),
        })
        .collect()
}

fn code_span(text: &str) -> String {
    let mut longest = 0;
    let mut current = 0;
    for character in text.chars() {
        if character == '`' {
            current += 1;
            longest = longest.max(current);
        } else {
            current = 0;
        }
    }
    let fence = "`".repeat(longest + 1);
    let padding = if text.starts_with('`') || text.ends_with('`') { " " } else { "" };
    format!("{fence}{padding}{text}{padding}{fence}")
}

fn destination(url: &str) -> String {
    if url.contains([' ', '(', ')', '<', '>']) {
        format!("<{}>", url.replace('<', "%3C").replace('>', "%3E"))
    } else {
        url.to_string()
    }
}

fn escape_text(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    for character in text.chars() {
        if "\\`*_[]<>#~|".contains(character) {
            output.push('\\');
        }
        output.push(character);
    }
    output
}

fn escape_line_starts(text: &str) -> String {
    text.split('\n').map(escape_line_start).collect::<Vec<_>>().join("\n")
}

fn escape_line_start(line: &str) -> String {
    if line.starts_with(['-', '+', '=']) {
        return format!("\\{line}");
    }
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    if digits > 0 && line[digits..].starts_with(['.', ')']) {
        return format!("{}\\{}", &line[..digits], &line[digits..]);
    }
    line.to_string()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::render;
    use crate::document::Document;

    fn markdown(content: serde_json::Value) -> String {
        render(&Document::from_value(&json!({ "type": "doc", "content": content })).expect("valid document"))
    }

    fn paragraph(value: &str) -> serde_json::Value {
        json!({ "type": "paragraph", "content": [{ "type": "text", "text": value }] })
    }

    fn item(value: &str) -> serde_json::Value {
        json!({ "type": "listItem", "content": [paragraph(value)] })
    }

    #[test]
    fn blocks_are_separated_by_blank_lines() {
        let output = markdown(json!([
            { "type": "heading", "attrs": { "level": 2 }, "content": [{ "type": "text", "text": "Title" }] },
            paragraph("One"),
            { "type": "paragraph" },
            paragraph("Two"),
            { "type": "horizontalRule" }
        ]));
        assert_eq!(output, "## Title\n\nOne\n\nTwo\n\n---");
    }

    #[test]
    fn special_characters_are_escaped() {
        assert_eq!(markdown(json!([paragraph("a *b* _c_ [d](e) <f> #g")])), "a \\*b\\* \\_c\\_ \\[d\\](e) \\<f\\> \\#g");
        assert_eq!(markdown(json!([paragraph("- not a list")])), "\\- not a list");
        assert_eq!(markdown(json!([paragraph("1. not a list")])), "1\\. not a list");
    }

    #[test]
    fn marks() {
        let output = markdown(json!([
            { "type": "paragraph", "content": [
                { "type": "text", "text": "bold ", "marks": [{ "type": "bold" }] },
                { "type": "text", "text": "italic", "marks": [{ "type": "italic" }] },
                { "type": "text", "text": " " },
                { "type": "text", "text": "gone", "marks": [{ "type": "strike" }] },
                { "type": "text", "text": " " },
                { "type": "text", "text": "under", "marks": [{ "type": "underline" }] },
                { "type": "text", "text": " " },
                { "type": "text", "text": "a`b", "marks": [{ "type": "code" }] }
            ] }
        ]));
        assert_eq!(output, "**bold** *italic* ~~gone~~ under ``a`b``");
    }

    #[test]
    fn links_group_across_marks() {
        let link = json!({ "type": "link", "attrs": { "href": "https://example.com/a_(b)" } });
        let output = markdown(json!([
            { "type": "paragraph", "content": [
                { "type": "text", "text": "see ", "marks": [link.clone()] },
                { "type": "text", "text": "this", "marks": [link, { "type": "bold" }] }
            ] }
        ]));
        assert_eq!(output, "[see **this**](<https://example.com/a_(b)>)");
    }

    #[test]
    fn hard_breaks_use_a_backslash() {
        let output = markdown(json!([
            { "type": "paragraph", "content": [
                { "type": "text", "text": "a" }, { "type": "hardBreak" }, { "type": "text", "text": "- b" }
            ] }
        ]));
        assert_eq!(output, "a\\\n\\- b");
    }

    #[test]
    fn nested_lists_and_numbering() {
        let output = markdown(json!([
            { "type": "orderedList", "attrs": { "start": 9 }, "content": [
                item("nine"),
                { "type": "listItem", "content": [
                    paragraph("ten"),
                    { "type": "bulletList", "content": [item("inner")] }
                ] }
            ] }
        ]));
        assert_eq!(output, "9. nine\n10. ten\n\n    - inner");
    }

    #[test]
    fn quotes_prefix_each_line() {
        let output = markdown(json!([
            { "type": "blockquote", "content": [paragraph("a"), paragraph("b")] }
        ]));
        assert_eq!(output, "> a\n>\n> b");
    }

    #[test]
    fn attributed_quotes_end_with_the_source() {
        let output = markdown(json!([
            { "type": "pullquote", "content": [paragraph("a"), { "type": "attribution", "content": [
                { "type": "text", "text": "Ava", "marks": [{ "type": "bold" }] }
            ] }] }
        ]));
        assert_eq!(output, "> a\n>\n> \u{2014} **Ava**");
    }

    #[test]
    fn code_blocks_choose_a_safe_fence() {
        let output = markdown(json!([
            { "type": "codeBlock", "attrs": { "language": "md" }, "content": [{ "type": "text", "text": "```\nx" }] },
            { "type": "codeBlock", "content": [{ "type": "text", "text": "*raw*" }] }
        ]));
        assert_eq!(output, "~~~~md\n```\nx\n~~~~\n\n```\n*raw*\n```");
    }

    #[test]
    fn figures() {
        let output = markdown(json!([
            { "type": "image", "attrs": { "src": "https://res.cloudinary.com/demo/a b.jpg", "caption": "A [cat]" } },
            { "type": "figure", "content": [
                { "type": "image", "attrs": { "src": "https://res.cloudinary.com/demo/b.jpg" } },
                { "type": "caption" }
            ] }
        ]));
        assert_eq!(
            output,
            "![A \\[cat\\]](<https://res.cloudinary.com/demo/a b.jpg>)\n\nA \\[cat\\]\n\n![](https://res.cloudinary.com/demo/b.jpg)"
        );
    }
}
