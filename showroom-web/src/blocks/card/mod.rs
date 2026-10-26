pub mod color;

use serde::{Deserialize, Deserializer};

use crate::theme::Color;

const MAX_TEXT: usize = 300;

pub fn clean_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(MAX_TEXT).collect()
}

pub fn card_color(color: Option<String>) -> Option<String> {
    color.as_deref().and_then(Color::parse).filter(|color| color::is_card_color(*color)).map(|color| color.to_string())
}

pub fn surface(color: Option<&str>) -> Option<Color> {
    color.and_then(Color::parse)
}

pub fn surface_style(surface: Option<Color>) -> Option<String> {
    surface.map(|surface| format!("--card-surface:{surface}"))
}

pub fn null_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    Option::<String>::deserialize(deserializer).map(Option::unwrap_or_default)
}

pub fn lenient_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    Ok(Option::<bool>::deserialize(deserializer).ok().flatten().unwrap_or(false))
}
