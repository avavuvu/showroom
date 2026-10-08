use super::{Color, Layout, Theme};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmailTheme {
    pub text: Color,
    pub border: Color,
    pub primary: Color,
    pub on_primary: Color,
    pub link: Color,
    pub surface: Color,
    pub surface_muted: Color,
    pub muted: Color,
    pub font_title: &'static str,
    pub font_body: &'static str,
    pub layout: Layout,
    pub text_px: u8,
    pub title_px: u8,
    pub heading_sizes: [u8; 3],
}

impl EmailTheme {
    pub fn heading_px(&self, level: u64) -> u8 {
        match level {
            1..=3 => self.heading_sizes[level as usize - 1],
            _ => self.text_px,
        }
    }
}

impl Default for EmailTheme {
    fn default() -> Self {
        Theme::default().email()
    }
}

pub(super) struct EmailSizes {
    pub text: u8,
    pub title: u8,
    pub headings: [u8; 3],
}

impl EmailSizes {
    pub(super) fn for_layout(layout: Layout) -> EmailSizes {
        match layout {
            Layout::Default | Layout::Centred => EmailSizes { text: 16, title: 28, headings: [22, 19, 18] },
            Layout::Compact => EmailSizes { text: 14, title: 14, headings: [14, 14, 14] },
            Layout::Letter => EmailSizes { text: 16, title: 16, headings: [22, 19, 18] },
        }
    }
}
