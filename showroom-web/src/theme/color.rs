use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Oklch {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
}

impl Oklch {
    pub fn to_color(self) -> Option<Color> {
        let (a, b) = (self.chroma * self.hue.cos(), self.chroma * self.hue.sin());
        let l = (self.lightness + 0.396_337_78 * a + 0.215_803_76 * b).powi(3);
        let m = (self.lightness - 0.105_561_346 * a - 0.063_854_17 * b).powi(3);
        let s = (self.lightness - 0.089_484_18 * a - 1.291_485_5 * b).powi(3);
        let linear = [
            4.076_741_7 * l - 3.307_711_6 * m + 0.230_969_94 * s,
            -1.268_438 * l + 2.609_757_4 * m - 0.341_319_38 * s,
            -0.004_196_086_3 * l - 0.703_418_6 * m + 1.707_614_7 * s,
        ];
        if linear.iter().any(|channel| !(-0.000_1..=1.000_1).contains(channel)) {
            return None;
        }
        let encode = |value: f32| {
            let value = value.clamp(0.0, 1.0);
            let encoded = if value <= 0.003_130_8 { 12.92 * value } else { 1.055 * value.powf(1.0 / 2.4) - 0.055 };
            (encoded * 255.0).round() as u8
        };
        Some(Color { red: encode(linear[0]), green: encode(linear[1]), blue: encode(linear[2]) })
    }
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

    pub fn is_dark(self) -> bool {
        self.luminance() < super::DARK_PAPER
    }

    pub fn contrast(self, other: Color) -> f32 {
        let (first, second) = (self.luminance(), other.luminance());
        (first.max(second) + 0.05) / (first.min(second) + 0.05)
    }

    pub fn oklch(self) -> Oklch {
        let decode = |value: u8| {
            let value = value as f32 / 255.0;
            if value <= 0.040_45 { value / 12.92 } else { ((value + 0.055) / 1.055).powf(2.4) }
        };
        let (r, g, b) = (decode(self.red), decode(self.green), decode(self.blue));
        let l = (0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b).cbrt();
        let m = (0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b).cbrt();
        let s = (0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b).cbrt();
        let lightness = 0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s;
        let a = 1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s;
        let b = 0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s;
        Oklch { lightness, chroma: a.hypot(b), hue: b.atan2(a) }
    }

    pub fn apca(self, background: Color) -> f32 {
        let (text, background) = (self.apca_luminance(), background.apca_luminance());
        let contrast = if background > text {
            (background.powf(0.56) - text.powf(0.57)) * 1.14
        } else {
            (background.powf(0.65) - text.powf(0.62)) * 1.14
        };

        match contrast {
            c if c.abs() < 0.1 => 0.0,
            c if c > 0.0 => (c - 0.027) * 100.0,
            c => (c + 0.027) * 100.0,
        }
    }

    fn apca_luminance(self) -> f32 {
        let linear = |value: u8| (value as f32 / 255.0).powf(2.4);
        let y = 0.2126729 * linear(self.red) + 0.7151522 * linear(self.green) + 0.0721750 * linear(self.blue);
        if y < 0.022 { y + (0.022 - y).powf(1.414) } else { y }
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
