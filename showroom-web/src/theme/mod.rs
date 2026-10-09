mod color;
mod email;
mod font;
mod layout;

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

pub use color::Color;
pub use email::EmailTheme;
pub use font::{Font, Fonts};
pub use layout::Layout;

use email::EmailSizes;

const MUTED: f32 = 0.33;
const SURFACE_MUTED: f32 = 0.07;
const DARK_PAPER: f32 = 0.18;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Theme {
    pub ink: Color,
    pub paper: Color,
    pub brand: Color,
    #[serde(default, skip_serializing_if = "Overrides::is_empty")]
    pub overrides: Overrides,
    #[serde(default, skip_serializing_if = "Fonts::is_default")]
    pub fonts: Fonts,
    #[serde(default, skip_serializing_if = "Layout::is_default")]
    pub layout: Layout,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Overrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_brand: Option<Color>,
}

impl Overrides {
    pub fn is_empty(&self) -> bool {
        *self == Overrides::default()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            ink: Color::hex(0x000000),
            paper: Color::hex(0xffffff),
            brand: Color::hex(0x92ca3a),
            overrides: Overrides::default(),
            fonts: Fonts::default(),
            layout: Layout::default(),
        }
    }
}

impl Theme {
    pub fn scheme(self) -> &'static str {
        if self.paper.luminance() < DARK_PAPER { "dark" } else { "light" }
    }

    pub fn on_brand(self) -> Color {
        self.overrides.on_brand.unwrap_or_else(|| {
            if self.ink.apca(self.brand).abs() >= self.paper.apca(self.brand).abs() { self.ink } else { self.paper }
        })
    }

    pub fn link(self) -> Color {
        self.overrides.link.unwrap_or(self.brand)
    }

    pub fn muted(self) -> Color {
        self.overrides.muted.unwrap_or_else(|| self.ink.mix(self.paper, MUTED))
    }

    pub fn css(self) -> String {
        let or_var = |color: Option<Color>, fallback: &str| color.map_or_else(|| fallback.to_string(), |color| color.to_string());
        format!(
            ":root{{--ink:{};--paper:{};--brand:{};--on-brand:{};--link:{};--muted:{};--font-title:{};--font-body:{};{}color-scheme:{}}}",
            self.ink,
            self.paper,
            self.brand,
            self.on_brand(),
            or_var(self.overrides.link, "var(--brand)"),
            or_var(self.overrides.muted, "var(--color-muted)"),
            self.fonts.title.stack(),
            self.fonts.body.stack(),
            self.layout.css(),
            self.scheme()
        )
    }

    pub fn email(self) -> EmailTheme {
        let sizes = EmailSizes::for_layout(self.layout);
        EmailTheme {
            text: self.ink,
            border: self.ink,
            primary: self.brand,
            on_primary: self.on_brand(),
            link: self.link(),
            surface: self.paper,
            surface_muted: self.ink.mix(self.paper, SURFACE_MUTED),
            muted: self.muted(),
            font_title: self.fonts.title.stack(),
            font_body: self.fonts.body.stack(),
            layout: self.layout,
            text_px: sizes.text,
            title_px: sizes.title,
            heading_sizes: sizes.headings,
        }
    }
}
