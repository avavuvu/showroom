use super::{
    Block, CodeBlockAttrs, Document, FigureAttrs, HeadingAttrs, ImageAttrs, Inline, ListItem, MAX_FALLBACK, Mark, NameAttrs, QuoteVariant, code_text,
    is_allowed_href,
    is_allowed_image_src,
};
use crate::blocks::music;

pub fn document(document: Document) -> Document {
    let content: Vec<Block> = document.content.into_iter().filter_map(top_level_block).collect();
    if content.is_empty() {
        return Document::default();
    }
    Document { content }
}

fn top_level_block(block: Block) -> Option<Block> {
    match block {
        Block::Music { attrs } => music::normalize(attrs).map(|attrs| Block::Music { attrs }),
        other => self::block(other),
    }
}

fn blocks(blocks: Vec<Block>) -> Vec<Block> {
    blocks.into_iter().filter_map(block).collect()
}

fn block(block: Block) -> Option<Block> {
    Some(match block {
        Block::Music { .. } => return None,
        Block::Paragraph { content } | Block::Caption { content } | Block::Attribution { content } => {
            Block::Paragraph { content: inlines(content) }
        }
        Block::Heading { attrs, content } => Block::Heading {
            attrs: HeadingAttrs { level: attrs.level.clamp(2, 4) },
            content: inlines(content),
        },
        Block::Blockquote { attrs, content } => match attrs.variant {
            QuoteVariant::Pull => quote(content, Vec::new()),
            QuoteVariant::Standard => Block::Blockquote { attrs: Default::default(), content: non_empty(blocks(content)) },
        },
        Block::Pullquote { content } => {
            let mut attribution = Vec::new();
            let mut body = Vec::new();
            for child in content {
                match child {
                    Block::Attribution { content } if attribution.is_empty() => attribution = content,
                    other => body.push(other),
                }
            }
            quote(body, attribution)
        }
        Block::BulletList { content } => Block::BulletList { content: list_items(content)? },
        Block::OrderedList { attrs, content } => Block::OrderedList { attrs, content: list_items(content)? },
        Block::CodeBlock { attrs, content } => {
            let text = code_text(&content);
            Block::CodeBlock {
                attrs: CodeBlockAttrs { language: attrs.language.filter(|language| is_valid_language(language)) },
                content: if text.is_empty() { Vec::new() } else { vec![Inline::Text { text, marks: Vec::new() }] },
            }
        }
        Block::HorizontalRule {} => Block::HorizontalRule {},
        Block::Image { attrs } => {
            let caption = attrs
                .caption
                .as_deref()
                .map(str::trim)
                .filter(|caption| !caption.is_empty())
                .map(|caption| vec![Inline::Text { text: caption.to_string(), marks: Vec::new() }])
                .unwrap_or_default();
            figure(FigureAttrs { width: attrs.width }, attrs, caption)?
        }
        Block::Figure { attrs, content } => {
            let mut image = None;
            let mut caption = Vec::new();
            for child in content {
                match child {
                    Block::Image { attrs } if image.is_none() => image = Some(attrs),
                    Block::Caption { content } => caption = content,
                    _ => {}
                }
            }
            figure(attrs, image?, caption)?
        }
    })
}

fn unquote(content: Vec<Block>) -> Vec<Block> {
    content
        .into_iter()
        .flat_map(|block| match block {
            Block::Blockquote { content, .. } => unquote(content),
            Block::Pullquote { content } => unquote(
                content
                    .into_iter()
                    .filter_map(|child| match child {
                        Block::Attribution { content } if content.is_empty() => None,
                        Block::Attribution { content } => Some(Block::Paragraph { content }),
                        other => Some(other),
                    })
                    .collect(),
            ),
            other => vec![other],
        })
        .collect()
}

fn quote(body: Vec<Block>, attribution: Vec<Inline>) -> Block {
    let mut content = non_empty(unquote(blocks(body)));
    content.push(Block::Attribution { content: inlines(attribution) });
    Block::Pullquote { content }
}

fn figure(attrs: FigureAttrs, image: ImageAttrs, caption: Vec<Inline>) -> Option<Block> {
    let src = image.src.trim().to_string();
    if !is_allowed_image_src(&src) {
        return None;
    }
    let image = ImageAttrs { src, title: image.title, public_id: image.public_id, width: Default::default(), caption: None };
    Some(Block::Figure {
        attrs,
        content: vec![Block::Image { attrs: image }, Block::Caption { content: trim_inlines(inlines(caption)) }],
    })
}


fn list_items(items: Vec<ListItem>) -> Option<Vec<ListItem>> {
    let items: Vec<ListItem> = items
        .into_iter()
        .map(|item| {
            let mut content = blocks(item.content);
            if !matches!(content.first(), Some(Block::Paragraph { .. })) {
                content.insert(0, Block::Paragraph { content: Vec::new() });
            }
            ListItem { content }
        })
        .collect();
    (!items.is_empty()).then_some(items)
}

fn non_empty(content: Vec<Block>) -> Vec<Block> {
    if content.is_empty() { vec![Block::Paragraph { content: Vec::new() }] } else { content }
}

fn trim_inlines(mut content: Vec<Inline>) -> Vec<Inline> {
    while matches!(content.last(), Some(Inline::HardBreak {})) {
        content.pop();
    }
    if let Some(Inline::Text { text, .. }) = content.first_mut() {
        *text = text.trim_start().to_string();
    }
    if let Some(Inline::Text { text, .. }) = content.last_mut() {
        *text = text.trim_end().to_string();
    }
    content.retain(|inline| !matches!(inline, Inline::Text { text, .. } if text.is_empty()));
    content
}

fn inlines(content: Vec<Inline>) -> Vec<Inline> {
    let mut result: Vec<Inline> = Vec::with_capacity(content.len());

    for inline in content {
        match inline {
            Inline::Text { text, marks } => {
                let marks = clean_marks(marks);
                if marks.contains(&Mark::Code {}) {
                    push_text(&mut result, &text, &marks);
                    continue;
                }
                let mut rest = text.as_str();
                while let Some((before, after)) = split_name(rest) {
                    push_text(&mut result, before, &marks);
                    result.push(Inline::Name { attrs: NameAttrs::default(), marks: marks.clone() });
                    rest = after;
                }
                push_text(&mut result, rest, &marks);
            }
            Inline::HardBreak {} => result.push(Inline::HardBreak {}),
            Inline::Name { attrs, marks } => result.push(Inline::Name {
                attrs: NameAttrs { fallback: clean_fallback(&attrs.fallback) },
                marks: clean_marks(marks).into_iter().filter(|mark| *mark != Mark::Code {}).collect(),
            }),
        }
    }

    result
}

fn clean_fallback(fallback: &str) -> String {
    fallback.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(MAX_FALLBACK).collect()
}

fn push_text(result: &mut Vec<Inline>, text: &str, marks: &[Mark]) {
    if text.is_empty() {
        return;
    }
    if let Some(Inline::Text { text: previous, marks: previous_marks }) = result.last_mut()
        && previous_marks == marks
    {
        previous.push_str(text);
        return;
    }
    result.push(Inline::Text { text: text.to_string(), marks: marks.to_vec() });
}

fn split_name(text: &str) -> Option<(&str, &str)> {
    let mut from = 0;
    while let Some(start) = text[from..].find("{{").map(|offset| from + offset) {
        let inner = &text[start + 2..];
        let end = inner.find("}}")?;
        if inner[..end].trim().eq_ignore_ascii_case("name") {
            return Some((&text[..start], &inner[end + 2..]));
        }
        from = start + 1;
    }
    None
}

fn clean_marks(marks: Vec<Mark>) -> Vec<Mark> {
    let mut result: Vec<Mark> = Vec::with_capacity(marks.len());
    for mark in marks {
        let mark = match mark {
            Mark::Link { mut attrs } => {
                attrs.href = attrs.href.trim().to_string();
                if !is_allowed_href(&attrs.href) {
                    continue;
                }
                Mark::Link { attrs }
            }
            other => other,
        };
        if result.iter().any(|existing| existing.rank() == mark.rank()) {
            continue;
        }
        result.push(mark);
    }
    result.sort_by_key(|mark| mark.rank());
    result
}

fn is_valid_language(language: &str) -> bool {
    !language.is_empty()
        && language.len() <= 32
        && language.chars().all(|character| character.is_ascii_alphanumeric() || "+#._-".contains(character))
}
