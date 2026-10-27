use crate::theme::{Font, Theme};

pub const EMAIL_PROFILE_PX: u32 = 48;
pub const SHARE_CARD_BASE: &str = "og-base";

#[derive(Clone, Copy)]
pub enum ImageSize {
    Hero,
    Card,
    Og,
    Banner,
    EmailPicture,
    Picture,
    Icon(u32),
}

impl ImageSize {
    pub fn dimensions(self) -> (u32, u32) {
        match self {
            ImageSize::Hero => (1600, 900),
            ImageSize::Card => (640, 360),
            ImageSize::Og => (1200, 630),
            ImageSize::Banner => (1200, 300),
            ImageSize::EmailPicture => (EMAIL_PROFILE_PX * 2, EMAIL_PROFILE_PX * 2),
            ImageSize::Picture => (256, 256),
            ImageSize::Icon(pixels) => (pixels, pixels),
        }
    }

    fn transform(self) -> String {
        let (width, height) = self.dimensions();
        match self {
            ImageSize::Hero | ImageSize::Card | ImageSize::Banner | ImageSize::Picture => format!("c_fill,g_auto,w_{width},h_{height},f_auto,q_auto"),
            ImageSize::Og => format!("c_fill,g_auto,w_{width},h_{height},f_jpg,q_auto"),
            ImageSize::EmailPicture => format!("c_fill,g_auto,w_{width},h_{height}"),
            ImageSize::Icon(_) => format!("c_fill,g_auto,w_{width},h_{height},f_png"),
        }
    }
}

pub fn cloudinary_url(cloud_name: &str, public_id: &str, size: ImageSize) -> String {
    format!("https://res.cloudinary.com/{cloud_name}/image/upload/{}/{public_id}", size.transform())
}

fn overlay_font(font: Font) -> &'static str {
    match font {
        Font::Arial | Font::Verdana => "Arial",
        Font::CourierNew | Font::LucidaConsole => "Courier",
        Font::Georgia | Font::Palatino => "Georgia",
        Font::Times => "Times",
        Font::Garamond => "EB%20Garamond",
    }
}

fn overlay_text(text: &str) -> String {
    let mut encoded = String::new();
    for character in text.chars() {
        match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => encoded.push(character),
            ',' => encoded.push_str("%252C"),
            '/' => encoded.push_str("%252F"),
            _ => {
                let mut buffer = [0; 4];
                for byte in character.encode_utf8(&mut buffer).bytes() {
                    encoded.push_str(&format!("%{byte:02X}"));
                }
            }
        }
    }
    encoded
}

fn hex(color: impl std::fmt::Display) -> String {
    color.to_string().trim_start_matches('#').to_string()
}

pub fn share_card(cloud_name: &str, title: &str, byline: &str, theme: Theme) -> String {
    let (width, height) = ImageSize::Og.dimensions();
    let title_font = overlay_font(theme.fonts.title);
    let body_font = overlay_font(theme.fonts.body);
    let ink = hex(theme.ink);
    let muted = hex(theme.muted());
    let title: String = title.chars().take(120).collect();
    let byline: String = byline.chars().take(80).collect();
    format!(
        "https://res.cloudinary.com/{cloud_name}/image/upload/c_scale,w_{width},h_{height}/e_colorize:100,co_rgb:{paper}/l_text:{title_font}_72_bold_line_spacing_-8:{title},co_rgb:{ink},w_1040,c_fit/fl_layer_apply,g_north_west,x_80,y_80/l_text:{body_font}_36:{byline},co_rgb:{muted},w_1040,c_fit/fl_layer_apply,g_south_west,x_80,y_80/f_jpg,q_auto/{SHARE_CARD_BASE}",
        paper = hex(theme.paper),
        title = overlay_text(&title),
        byline = overlay_text(&byline),
    )
}
