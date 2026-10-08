mod inline;
mod normalize;

pub use inline::{InlinePiece, inline_tree};

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
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadingAttrs {
    #[serde(default = "default_heading_level", deserialize_with = "lenient_level")]
    pub level: u8,
}

impl Default for HeadingAttrs {
    fn default() -> Self {
        Self { level: default_heading_level() }
    }
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
            eprintln!("[error] stored document is not valid: {error}");
            Self::default()
        })
    }

    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
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
        })
        .collect()
}

pub fn code_text(content: &[Inline]) -> String {
    content
        .iter()
        .map(|inline| match inline {
            Inline::Text { text, .. } => text.as_str(),
            Inline::HardBreak {} => "\n",
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
