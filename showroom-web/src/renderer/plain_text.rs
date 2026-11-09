use std::borrow::Cow;

use crate::blocks::youtube::WATCH_LABEL;
use crate::document::{Block, Document, Inline, InlinePiece, ListItem, Mark, MusicAttrs, MusicKind, YoutubeAttrs, code_text, figure_parts, inline_tree, quote_parts};
use crate::renderer::{Links, personalize::escape};

#[derive(Clone, Copy)]
enum Output {
    Template,
    Text(Links),
}

impl Output {
    fn escape(self, text: &str) -> Cow<'_, str> {
        match self {
            Output::Template => escape(text),
            Output::Text(_) => Cow::Borrowed(text),
        }
    }
}

pub fn render(document: &Document) -> String {
    blocks(document.blocks(), Output::Template)
}

pub fn text(document: &Document, links: Links) -> String {
    blocks(document.blocks(), Output::Text(links))
}

fn blocks(blocks: &[Block], output: Output) -> String {
    blocks.iter().map(|item| block(item, output)).filter(|text| !text.is_empty()).collect::<Vec<_>>().join("\n\n")
}

fn block(block: &Block, output: Output) -> String {
    let text = block_text(block, output);
    match output {
        Output::Template if block.requires_name() && !text.is_empty() => format!("{{{{#if name}}}}{text}{{{{/if}}}}"),
        Output::Text(_) if block.requires_name() => String::new(),
        _ => text,
    }
}

fn block_text(block: &Block, output: Output) -> String {
    match block {
        Block::Paragraph { content }
        | Block::Heading { content, .. }
        | Block::Caption { content }
        | Block::Attribution { content } => inlines(content, output),
        Block::Blockquote { content, .. } => content_lines(&blocks(content, output), "> ", ">"),
        Block::Pullquote { content } => {
            let (body, attribution) = quote_parts(content);
            let mut text = blocks(body, output);
            if !attribution.is_empty() {
                text.push_str("\n\n\u{2014} ");
                text.push_str(&inlines(attribution, output));
            }
            content_lines(&text, "> ", ">")
        }
        Block::BulletList { content } => content.iter().map(|item| list_item("-", item, output)).collect::<Vec<_>>().join("\n"),
        Block::OrderedList { attrs, content } => content
            .iter()
            .enumerate()
            .map(|(index, item)| list_item(&format!("{}.", attrs.start as usize + index), item, output))
            .collect::<Vec<_>>()
            .join("\n"),
        Block::CodeBlock { content, .. } => output.escape(&code_text(content)).into_owned(),
        Block::HorizontalRule {} => "---".to_string(),
        Block::Image { attrs } => output.escape(&attrs.src).into_owned(),
        Block::Figure { content, .. } => match figure_parts(content) {
            Some((image, caption)) if !caption.is_empty() => format!("{} ({})", inlines(caption, output), output.escape(&image.src)),
            Some((image, _)) => output.escape(&image.src).into_owned(),
            None => String::new(),
        },
        Block::Music { attrs } => music(attrs, output),
        Block::Youtube { attrs } => youtube(attrs, output),
        Block::Boxout { content, .. } => format!("---\n\n{}\n\n---", blocks(content, output)),
    }
}

fn youtube(attrs: &YoutubeAttrs, output: Output) -> String {
    format!("{WATCH_LABEL}: {}", output.escape(&attrs.watch_url()))
}

fn music(attrs: &MusicAttrs, output: Output) -> String {
    let mut heading = [attrs.title.as_str(), attrs.artist.as_str()]
        .into_iter()
        .filter(|text| !text.is_empty())
        .map(|text| output.escape(text).into_owned())
        .collect::<Vec<_>>()
        .join(" \u{2014} ");
    if attrs.kind == MusicKind::Album && !heading.is_empty() {
        heading.push_str(" (album)");
    }
    std::iter::once(heading)
        .filter(|text| !text.is_empty())
        .chain(attrs.links.iter().map(|(service, link)| format!("{}: {}", service.label(), output.escape(&link.url))))
        .collect::<Vec<_>>()
        .join("\n")
}

fn content_lines(text: &str, prefix: &str, empty: &str) -> String {
    text.lines()
        .map(|line| if line.is_empty() { empty.to_string() } else { format!("{prefix}{line}") })
        .collect::<Vec<_>>()
        .join("\n")
}

fn list_item(marker: &str, item: &ListItem, output: Output) -> String {
    let body = blocks(&item.content, output);
    let indent = " ".repeat(marker.len() + 1);
    let mut lines = body.lines();
    let mut rendered = format!("{marker} {}", lines.next().unwrap_or(""));
    for line in lines {
        rendered.push('\n');
        if !line.is_empty() {
            rendered.push_str(&indent);
            rendered.push_str(line);
        }
    }
    rendered.trim_end().to_string()
}

fn inlines(content: &[Inline], output: Output) -> String {
    pieces(&inline_tree(content), output)
}

fn pieces(pieces: &[InlinePiece], output: Output) -> String {
    pieces
        .iter()
        .map(|piece| match piece {
            InlinePiece::Text(text) => output.escape(text).into_owned(),
            InlinePiece::Break => "\n".to_string(),
            InlinePiece::Name(fallback) => match output {
                Output::Template => format!("{{{{#if name}}}}{{{{{{name}}}}}}{{{{else}}}}{}{{{{/if}}}}", escape(fallback)),
                Output::Text(_) => fallback.to_string(),
            },
            InlinePiece::Marked(Mark::Link { .. }, children) if matches!(output, Output::Text(Links::Inert)) => self::pieces(children, output),
            InlinePiece::Marked(Mark::Link { attrs }, children) => {
                let text = self::pieces(children, output);
                let href = output.escape(&attrs.href);
                let shown = href.strip_prefix("mailto:").or_else(|| href.strip_prefix("tel:")).unwrap_or(&href);
                if text.trim() == shown || text.trim() == href {
                    text
                } else {
                    format!("{text} ({shown})")
                }
            }
            InlinePiece::Marked(_, children) => self::pieces(children, output),
        })
        .collect()
}
