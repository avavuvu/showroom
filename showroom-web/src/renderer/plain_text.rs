use crate::document::{Block, Document, Inline, InlinePiece, ListItem, Mark, MusicAttrs, MusicKind, code_text, figure_parts, inline_tree, quote_parts};
use crate::renderer::personalize::escape;

pub fn render(document: &Document) -> String {
    blocks(document.blocks())
}

fn blocks(blocks: &[Block]) -> String {
    blocks.iter().map(block).filter(|text| !text.is_empty()).collect::<Vec<_>>().join("\n\n")
}

fn block(block: &Block) -> String {
    let text = block_text(block);
    if block.requires_name() && !text.is_empty() { format!("{{{{#if name}}}}{text}{{{{/if}}}}") } else { text }
}

fn block_text(block: &Block) -> String {
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
        Block::CodeBlock { content, .. } => escape(&code_text(content)).into_owned(),
        Block::HorizontalRule {} => "---".to_string(),
        Block::Image { attrs } => escape(&attrs.src).into_owned(),
        Block::Figure { content, .. } => match figure_parts(content) {
            Some((image, caption)) if !caption.is_empty() => format!("{} ({})", inlines(caption), escape(&image.src)),
            Some((image, _)) => escape(&image.src).into_owned(),
            None => String::new(),
        },
        Block::Music { attrs } => music(attrs),
    }
}

fn music(attrs: &MusicAttrs) -> String {
    let mut heading = [attrs.title.as_str(), attrs.artist.as_str()]
        .into_iter()
        .filter(|text| !text.is_empty())
        .map(|text| escape(text).into_owned())
        .collect::<Vec<_>>()
        .join(" \u{2014} ");
    if attrs.kind == MusicKind::Album && !heading.is_empty() {
        heading.push_str(" (album)");
    }
    std::iter::once(heading)
        .filter(|text| !text.is_empty())
        .chain(attrs.links.iter().map(|(service, link)| format!("{}: {}", service.label(), escape(&link.url))))
        .collect::<Vec<_>>()
        .join("\n")
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
            InlinePiece::Text(text) => escape(text).into_owned(),
            InlinePiece::Break => "\n".to_string(),
            InlinePiece::Name(fallback) => format!("{{{{#if name}}}}{{{{{{name}}}}}}{{{{else}}}}{}{{{{/if}}}}", escape(fallback)),
            InlinePiece::Marked(Mark::Link { attrs }, children) => {
                let text = self::pieces(children);
                let href = escape(&attrs.href);
                let shown = href.strip_prefix("mailto:").or_else(|| href.strip_prefix("tel:")).unwrap_or(&href);
                if text.trim() == shown || text.trim() == href {
                    text
                } else {
                    format!("{text} ({shown})")
                }
            }
            InlinePiece::Marked(_, children) => self::pieces(children),
        })
        .collect()
}
