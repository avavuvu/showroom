mod inline;
pub mod merge;
mod normalize;

pub use crate::blocks::music::{MusicAttrs, MusicKind};
pub use crate::blocks::youtube::YoutubeAttrs;
pub use crate::blocks::boxout::{BoxoutAttrs, BoxoutStyle, BoxoutWidth};
pub use inline::{InlinePiece, inline_tree, join_words, words};

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "doc")]
pub struct Document {
    #[serde(default)]
    pub content: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Block {
    Paragraph {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        content: Vec<Inline>,
    },
    Heading {
        #[serde(default)]
        attrs: HeadingAttrs,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        content: Vec<Inline>,
    },
    Blockquote {
        #[serde(default, skip_serializing)]
        attrs: BlockquoteAttrs,
        #[serde(default)]
        content: Vec<Block>,
    },
    #[serde(alias = "quote")]
    Pullquote {
        #[serde(default)]
        content: Vec<Block>,
    },
    Attribution {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        content: Vec<Inline>,
    },
    BulletList {
        #[serde(default)]
        content: Vec<ListItem>,
    },
    OrderedList {
        #[serde(default)]
        attrs: OrderedListAttrs,
        #[serde(default)]
        content: Vec<ListItem>,
    },
    CodeBlock {
        #[serde(default)]
        attrs: CodeBlockAttrs,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        content: Vec<Inline>,
    },
    HorizontalRule {},
    Figure {
        #[serde(default)]
        attrs: FigureAttrs,
        #[serde(default)]
        content: Vec<Block>,
    },
    Image {
        attrs: ImageAttrs,
    },
    Caption {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        content: Vec<Inline>,
    },
    Music {
        #[serde(default)]
        attrs: MusicAttrs,
    },
    Youtube {
        #[serde(default)]
        attrs: YoutubeAttrs,
    },
    Boxout {
        #[serde(default)]
        attrs: BoxoutAttrs,
        #[serde(default)]
        content: Vec<Block>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "listItem")]
pub struct ListItem {
    #[serde(default)]
    pub content: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Inline {
    Text {
        #[serde(deserialize_with = "null_string")]
        text: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        marks: Vec<Mark>,
    },
    HardBreak {},
    Name {
        #[serde(default)]
        attrs: NameAttrs,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        marks: Vec<Mark>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NameAttrs {
    #[serde(default, deserialize_with = "null_string")]
    pub fallback: String,
}

impl NameAttrs {
    pub fn with_fallback(fallback: &str) -> Self {
        Self { fallback: fallback.to_string() }
    }
}

pub const NAME_TOKEN: &str = "{{name}}";
pub const MAX_FALLBACK: usize = 40;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Mark {
    Link { attrs: LinkAttrs },
    Bold {},
    Italic {},
    Underline {},
    Strike {},
    Code {},
}

impl Mark {
    fn rank(&self) -> u8 {
        match self {
            Mark::Link { .. } => 0,
            Mark::Bold {} => 1,
            Mark::Italic {} => 2,
            Mark::Underline {} => 3,
            Mark::Strike {} => 4,
            Mark::Code {} => 5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkAttrs {
    #[serde(default, deserialize_with = "null_string")]
    pub href: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadingAttrs {
    #[serde(default = "default_heading_level", deserialize_with = "lenient_level")]
    pub level: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Default for HeadingAttrs {
    fn default() -> Self {
        Self { level: default_heading_level(), id: None }
    }
}

pub fn is_valid_section_id(id: &str) -> bool {
    (1..=32).contains(&id.len()) && id.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
pub struct BlockquoteAttrs {
    #[serde(default, deserialize_with = "lenient_variant")]
    pub variant: QuoteVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuoteVariant {
    #[default]
    Standard,
    Pull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderedListAttrs {
    #[serde(default = "default_start", deserialize_with = "lenient_start")]
    pub start: u32,
}

impl Default for OrderedListAttrs {
    fn default() -> Self {
        Self { start: default_start() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CodeBlockAttrs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FigureAttrs {
    #[serde(default, deserialize_with = "lenient_width")]
    pub width: ImageWidth,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageAttrs {
    #[serde(default, deserialize_with = "null_string")]
    pub src: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, rename = "publicId", skip_serializing_if = "Option::is_none")]
    pub public_id: Option<String>,
    #[serde(default, skip_serializing, deserialize_with = "lenient_width")]
    pub width: ImageWidth,
    #[serde(default, skip_serializing)]
    pub caption: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageWidth {
    #[default]
    Normal,
    Full,
}

impl ImageWidth {
    pub fn as_str(self) -> &'static str {
        match self {
            ImageWidth::Normal => "normal",
            ImageWidth::Full => "full",
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self { content: vec![Block::Paragraph { content: Vec::new() }] }
    }
}

impl Document {
    pub fn from_value(value: &Value) -> Result<Self, serde_json::Error> {
        Document::deserialize(value).map(normalize::document)
    }

    pub fn from_stored(value: &Value) -> Self {
        Self::from_value(value).unwrap_or_else(|error| {
            tracing::error!(%error, "a stored document is not valid");
            Self::default()
        })
    }

    pub fn greeting() -> Self {
        let text = |text: &str| Inline::Text { text: text.to_string(), marks: Vec::new() };
        Self {
            content: vec![
                Block::Paragraph { content: vec![text("Hi "), Inline::Name { attrs: NameAttrs::with_fallback("friend"), marks: Vec::new() }, text(",")] },
                Block::Paragraph { content: Vec::new() },
            ],
        }
    }

    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }

    pub fn section_span(&self, id: &str) -> Option<std::ops::Range<usize>> {
        let blocks = self.blocks();
        let (start, level) = blocks.iter().enumerate().find_map(|(index, block)| match block {
            Block::Heading { attrs, .. } if attrs.id.as_deref() == Some(id) => Some((index, attrs.level)),
            _ => None,
        })?;
        let end = blocks[start + 1..]
            .iter()
            .position(|block| matches!(block, Block::Heading { attrs, .. } if attrs.level <= level))
            .map_or(blocks.len(), |offset| start + 1 + offset);
        Some(start..end)
    }

    pub fn scope(&self, section_id: Option<&str>) -> Option<std::ops::Range<usize>> {
        match section_id {
            Some(id) => self.section_span(id),
            None => Some(0..self.blocks().len()),
        }
    }

    pub fn heading_before(&self, index: usize) -> Option<String> {
        self.blocks()[..index].iter().rev().find_map(|block| match block {
            Block::Heading { attrs, .. } => attrs.id.clone(),
            _ => None,
        })
    }

    pub fn restore_index(&self, after_section_id: Option<&str>) -> usize {
        let blocks = self.blocks();
        let start = match after_section_id {
            Some(id) => match blocks.iter().position(|block| matches!(block, Block::Heading { attrs, .. } if attrs.id.as_deref() == Some(id))) {
                Some(index) => index + 1,
                None => return blocks.len(),
            },
            None => 0,
        };
        blocks[start..]
            .iter()
            .position(|block| matches!(block, Block::Heading { .. }))
            .map_or(blocks.len(), |offset| start + offset)
    }

    pub fn trimmed(&self) -> Self {
        Self { content: self.blocks().to_vec() }
    }

    pub fn blocks(&self) -> &[Block] {
        let end = self
            .content
            .iter()
            .rposition(|block| !block.is_empty_paragraph())
            .map_or(0, |index| index + 1);
        &self.content[..end]
    }
}

impl Block {
    pub fn inline_content(&self) -> Option<&[Inline]> {
        match self {
            Block::Paragraph { content } | Block::Heading { content, .. } | Block::Caption { content } | Block::Attribution { content } => Some(content),
            _ => None,
        }
    }

    pub fn same_text_kind(&self, other: &Block) -> bool {
        match (self, other) {
            (Block::Heading { attrs, .. }, Block::Heading { attrs: other, .. }) => attrs.level == other.level,
            _ => std::mem::discriminant(self) == std::mem::discriminant(other) && self.inline_content().is_some(),
        }
    }

    pub fn with_inline_content(&self, content: Vec<Inline>) -> Block {
        match self {
            Block::Paragraph { .. } => Block::Paragraph { content },
            Block::Heading { attrs, .. } => Block::Heading { attrs: attrs.clone(), content },
            Block::Caption { .. } => Block::Caption { content },
            Block::Attribution { .. } => Block::Attribution { content },
            _ => self.clone(),
        }
    }

    pub fn is_card(&self) -> bool {
        match self {
            Block::Music { .. } | Block::Youtube { .. } | Block::Figure { .. } | Block::Image { .. } | Block::HorizontalRule {} | Block::Boxout { .. } => true,
            Block::Paragraph { .. }
            | Block::Heading { .. }
            | Block::Blockquote { .. }
            | Block::Pullquote { .. }
            | Block::Attribution { .. }
            | Block::BulletList { .. }
            | Block::OrderedList { .. }
            | Block::CodeBlock { .. }
            | Block::Caption { .. } => false,
        }
    }

    pub fn media_keys(&self) -> Vec<String> {
        match self {
            Block::Image { attrs } => vec![format!("image:{}", attrs.src)],
            Block::Music { attrs } => vec![format!("music:{}", attrs.artwork.as_deref().unwrap_or_default())],
            Block::Youtube { attrs } => vec![format!("youtube:{}", attrs.id)],
            Block::Figure { content, .. } | Block::Blockquote { content, .. } | Block::Pullquote { content } | Block::Boxout { content, .. } => {
                content.iter().flat_map(Block::media_keys).collect()
            }
            Block::BulletList { content } | Block::OrderedList { content, .. } => {
                content.iter().flat_map(|item| item.content.iter().flat_map(Block::media_keys)).collect()
            }
            _ => Vec::new(),
        }
    }

    pub fn requires_name(&self) -> bool {
        match self {
            Block::Paragraph { content } | Block::Heading { content, .. } | Block::Caption { content } | Block::Attribution { content } => {
                requires_name(content)
            }
            _ => false,
        }
    }

    fn is_empty_paragraph(&self) -> bool {
        matches!(self, Block::Paragraph { content } if content.is_empty())
    }
}

pub fn figure_parts(content: &[Block]) -> Option<(&ImageAttrs, &[Inline])> {
    let image = content.iter().find_map(|block| match block {
        Block::Image { attrs } => Some(attrs),
        _ => None,
    })?;
    let caption = content
        .iter()
        .find_map(|block| match block {
            Block::Caption { content } => Some(content.as_slice()),
            _ => None,
        })
        .unwrap_or(&[]);
    Some((image, caption))
}

pub fn quote_parts(content: &[Block]) -> (&[Block], &[Inline]) {
    match content.split_last() {
        Some((Block::Attribution { content: attribution }, body)) => (body, attribution),
        _ => (content, &[]),
    }
}

pub fn inline_text(content: &[Inline]) -> String {
    content
        .iter()
        .map(|inline| match inline {
            Inline::Text { text, .. } => text.as_str(),
            Inline::HardBreak {} => " ",
            Inline::Name { .. } => NAME_TOKEN,
        })
        .collect()
}

pub fn alt_text(content: &[Inline]) -> String {
    let text: String = content
        .iter()
        .map(|inline| match inline {
            Inline::Text { text, .. } => text.as_str(),
            Inline::HardBreak {} => " ",
            Inline::Name { attrs, .. } => attrs.fallback.as_str(),
        })
        .collect();
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn requires_name(content: &[Inline]) -> bool {
    content.iter().any(|inline| matches!(inline, Inline::Name { attrs, .. } if attrs.fallback.is_empty()))
}

pub fn code_text(content: &[Inline]) -> String {
    content
        .iter()
        .map(|inline| match inline {
            Inline::Text { text, .. } => text.as_str(),
            Inline::HardBreak {} => "\n",
            Inline::Name { .. } => NAME_TOKEN,
        })
        .collect()
}

pub fn is_allowed_href(href: &str) -> bool {
    match url::Url::parse(href) {
        Ok(url) => match url.scheme() {
            "http" | "https" => url.host_str().is_some_and(|host| !host.is_empty()),
            "mailto" | "tel" => !url.path().is_empty(),
            _ => false,
        },
        Err(_) => false,
    }
}

pub fn is_allowed_image_src(src: &str) -> bool {
    url::Url::parse(src).is_ok_and(|url| url.scheme() == "https" && url.host_str().is_some())
}

fn default_heading_level() -> u8 {
    2
}

fn default_start() -> u32 {
    1
}

fn null_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    Option::<String>::deserialize(deserializer).map(Option::unwrap_or_default)
}

fn lenient_level<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u8, D::Error> {
    let level = Option::<i64>::deserialize(deserializer)?.unwrap_or(default_heading_level().into());
    Ok(level.clamp(1, 6) as u8)
}

fn lenient_start<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    let start = Option::<i64>::deserialize(deserializer)?.unwrap_or(default_start().into());
    Ok(start.clamp(0, 1_000_000) as u32)
}

fn lenient_variant<'de, D: Deserializer<'de>>(deserializer: D) -> Result<QuoteVariant, D::Error> {
    Ok(match Option::<String>::deserialize(deserializer)?.as_deref() {
        Some("pull") => QuoteVariant::Pull,
        _ => QuoteVariant::Standard,
    })
}

fn lenient_width<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ImageWidth, D::Error> {
    Ok(match Option::<String>::deserialize(deserializer)?.as_deref() {
        Some("full") => ImageWidth::Full,
        _ => ImageWidth::Normal,
    })
}
