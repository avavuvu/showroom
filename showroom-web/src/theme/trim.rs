use serde::{Deserialize, Serialize};

use super::Color;

pub const WOOD_COLOR: Color = Color::hex(0xb87c3d);
pub const WOOD_EMAIL_PATH: &str = "/assets/wood.jpg";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum Trim {
    #[default]
    Wood,
    Plain,
    Color(Color),
}

impl Trim {
    pub fn key(self) -> String {
        match self {
            Trim::Wood => "wood".to_string(),
            Trim::Plain => "none".to_string(),
            Trim::Color(color) => color.to_string(),
        }
    }

    pub fn parse(text: &str) -> Option<Trim> {
        match text.trim() {
            "wood" => Some(Trim::Wood),
            "none" => Some(Trim::Plain),
            other => Color::parse(other).map(Trim::Color),
        }
    }

    pub fn is_default(&self) -> bool {
        *self == Trim::Wood
    }

    pub fn color(self) -> Option<Color> {
        match self {
            Trim::Color(color) => Some(color),
            Trim::Wood | Trim::Plain => None,
        }
    }

    pub(super) fn css(self) -> String {
        match self {
            Trim::Wood => String::new(),
            Trim::Plain => "--trim-material:linear-gradient(var(--color-primary) 0 0);".to_string(),
            Trim::Color(color) => format!("--trim-material:linear-gradient({color} 0 0);"),
        }
    }
}

impl From<String> for Trim {
    fn from(text: String) -> Self {
        Trim::parse(&text).unwrap_or_default()
    }
}

impl From<Trim> for String {
    fn from(trim: Trim) -> Self {
        trim.key()
    }
}
