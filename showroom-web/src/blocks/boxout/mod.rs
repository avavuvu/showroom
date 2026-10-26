mod editor;

pub use editor::BoxoutEditor;

use serde::{Deserialize, Deserializer, Serialize};

#[boutique::ts]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BoxoutAttrs {
    #[serde(default, deserialize_with = "lenient_style")]
    pub style: BoxoutStyle,
    #[serde(default, deserialize_with = "lenient_width")]
    pub width: BoxoutWidth,
}

#[boutique::ts]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum BoxoutStyle {
    #[default]
    Bordered,
    Brand,
    Trim,
}

#[boutique::ts]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum BoxoutWidth {
    #[default]
    Wide,
    Text,
    Left,
    Right,
}

impl BoxoutStyle {
    pub fn as_str(self) -> &'static str {
        match self {
            BoxoutStyle::Bordered => "bordered",
            BoxoutStyle::Brand => "brand",
            BoxoutStyle::Trim => "trim",
        }
    }
}

impl BoxoutWidth {
    pub fn as_str(self) -> &'static str {
        match self {
            BoxoutWidth::Wide => "wide",
            BoxoutWidth::Text => "text",
            BoxoutWidth::Left => "left",
            BoxoutWidth::Right => "right",
        }
    }

    pub fn is_side(self) -> bool {
        matches!(self, BoxoutWidth::Left | BoxoutWidth::Right)
    }
}

fn lenient_style<'de, D: Deserializer<'de>>(deserializer: D) -> Result<BoxoutStyle, D::Error> {
    Ok(match Option::<String>::deserialize(deserializer).ok().flatten().as_deref() {
        Some("brand") => BoxoutStyle::Brand,
        Some("trim") => BoxoutStyle::Trim,
        _ => BoxoutStyle::Bordered,
    })
}

fn lenient_width<'de, D: Deserializer<'de>>(deserializer: D) -> Result<BoxoutWidth, D::Error> {
    Ok(match Option::<String>::deserialize(deserializer).ok().flatten().as_deref() {
        Some("text") => BoxoutWidth::Text,
        Some("left") => BoxoutWidth::Left,
        Some("right") => BoxoutWidth::Right,
        _ => BoxoutWidth::Wide,
    })
}
