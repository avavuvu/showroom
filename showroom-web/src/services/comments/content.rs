use maud::Markup;
use serde_json::Value;

use crate::{
    document::{Block, Document, Inline, ListItem, Mark},
    renderer::{Links, html, plain_text},
};

use super::MAX_BODY;

#[derive(Clone, Debug)]
pub struct Content(Document);

#[derive(Clone, Copy, Debug)]
pub enum Invalid {
    Empty,
    TooLong,
    Unsupported,
}

impl Content {
    pub fn from_form(content: Option<&str>, body: &str) -> Result<Self, Invalid> {
        let document = match content {
            Some(content) => serde_json::from_str::<Value>(content)
                .ok()
                .and_then(|value| Document::from_value(&value).ok())
                .ok_or(Invalid::Unsupported)?,
            None => from_text(body),
        };
        if !document.content.iter().all(is_allowed_block) {
            return Err(Invalid::Unsupported);
        }

        let content = Self(document);
        let text = content.text();
        if text.trim().is_empty() {
            Err(Invalid::Empty)
        } else if text.chars().count() as u64 > MAX_BODY {
            Err(Invalid::TooLong)
        } else {
            Ok(content)
        }
    }

    pub fn from_stored(value: &Value) -> Self {
        Self(Document::from_stored(value))
    }

    pub fn to_value(&self) -> Value {
        self.0.to_value()
    }

    pub fn html(&self) -> Markup {
        html::render_with(&self.0, Links::UserGenerated)
    }

    pub fn text(&self) -> String {
        plain_text::text(&self.0, Links::UserGenerated)
    }

    pub fn email_html(&self) -> Markup {
        html::render_with(&self.0, Links::Inert)
    }

    pub fn email_text(&self) -> String {
        plain_text::text(&self.0, Links::Inert)
    }
}

fn from_text(body: &str) -> Document {
    let mut paragraphs: Vec<Vec<&str>> = Vec::new();
    let mut is_separated = true;
    for line in body.lines() {
        if line.trim().is_empty() {
            is_separated = true;
        } else if is_separated {
            paragraphs.push(vec![line]);
            is_separated = false;
        } else if let Some(paragraph) = paragraphs.last_mut() {
            paragraph.push(line);
        }
    }

    let paragraph = |lines: Vec<&str>| Block::Paragraph {
        content: lines
            .into_iter()
            .enumerate()
            .flat_map(|(index, line)| {
                let line_break = (index > 0).then_some(Inline::HardBreak {});
                line_break.into_iter().chain([Inline::Text { text: line.to_string(), marks: Vec::new() }])
            })
            .collect(),
    };
    Document { content: paragraphs.into_iter().map(paragraph).collect() }
}

fn is_allowed_inline(inline: &Inline) -> bool {
    match inline {
        Inline::Text { marks, .. } => marks.iter().all(|mark| matches!(mark, Mark::Bold {} | Mark::Italic {} | Mark::Code {} | Mark::Link { .. })),
        Inline::HardBreak {} => true,
        Inline::Name { .. } => false,
    }
}

fn is_allowed_item(item: &ListItem) -> bool {
    item.content.iter().all(is_allowed_block)
}

fn is_allowed_block(block: &Block) -> bool {
    match block {
        Block::Paragraph { content } => content.iter().all(is_allowed_inline),
        Block::Blockquote { content, .. } => content.iter().all(is_allowed_block),
        Block::BulletList { content } | Block::OrderedList { content, .. } => content.iter().all(is_allowed_item),
        _ => false,
    }
}
