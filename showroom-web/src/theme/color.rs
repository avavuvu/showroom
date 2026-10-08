use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const fn hex(rgb: u32) -> Color {
        Color { red: (rgb >> 16) as u8, green: (rgb >> 8) as u8, blue: rgb as u8 }
    }

    pub fn parse(text: &str) -> Option<Color> {
        let digits = text.trim().strip_prefix('#')?;
        if digits.len() != 6 || !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        u32::from_str_radix(digits, 16).ok().map(Color::hex)
    }

    pub fn mix(self, other: Color, amount: f32) -> Color {
        let amount = amount.clamp(0.0, 1.0);
        let channel = |x: u8, y: u8| (x as f32 * amount + y as f32 * (1.0 - amount)).round() as u8;
        Color {
            red: channel(self.red, other.red),
            green: channel(self.green, other.green),
            blue: channel(self.blue, other.blue),
        }
    }

    pub fn luminance(self) -> f32 {
        let linear = |value: u8| {
            let value = value as f32 / 255.0;
            if value <= 0.03928 { value / 12.92 } else { ((value + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * linear(self.red) + 0.7152 * linear(self.green) + 0.0722 * linear(self.blue)
    }

    pub fn contrast(self, other: Color) -> f32 {
        let (a, b) = (self.luminance(), other.luminance());
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

impl Serialize for Color {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Color, D::Error> {
        let text = String::deserialize(deserializer)?;
        Color::parse(&text).ok_or_else(|| serde::de::Error::custom(format!("invalid color {text:?}")))
    }
}
