use boutique::html;
use maud::{Markup, PreEscaped};

use crate::document::{
    Block, BoxoutAttrs, BoxoutStyle, BoxoutWidth, Document, ImageAttrs, ImageWidth, Inline, InlinePiece, ListItem, Mark, MusicAttrs, MusicKind,
    YoutubeAttrs, alt_text, code_text,
    figure_parts, inline_tree, quote_parts, requires_name,
};
use crate::blocks::card::color::WHITE;
use crate::blocks::music::listen_label;
use crate::blocks::youtube::WATCH_LABEL;
use crate::renderer::personalize;
use crate::state::Urls;
use crate::theme::{Color, EmailTheme, Layout, Trim, WOOD_COLOR, WOOD_EMAIL_PATH};

const MUSIC_ARTWORK_PX: u8 = 96;
const MUSIC_ICON_PX: u8 = 22;
const MUSIC_PADDING_PX: u8 = 12;
const MUSIC_GAP_PX: u8 = 16;
const CARD_PADDING_PX: u8 = 12;
const BOXOUT_SIDE_PX: u16 = 254;
const WRAP_INSET_PX: u8 = 24;
const BOXOUT_GAP_PX: u16 = 16;
const BOXOUT_SIDE_IMAGE_PX: u16 = BOXOUT_SIDE_PX - BOXOUT_GAP_PX - 2 * BOXOUT_PADDING_PX as u16 - 2;
const BOXOUT_PADDING_PX: u8 = 16;
const BOXOUT_TRIM_PX: u8 = 14;
const CLEAR: &str = r#"<br clear="all">"#;

pub enum EmailBlock {
    Content(String),
    Wide(String),
    FullWidthImage { src: String, alt: String, caption: Option<String> },
}

impl EmailBlock {
    pub fn map(self, f: impl Fn(&str) -> String) -> EmailBlock {
        match self {
            EmailBlock::Content(html) => EmailBlock::Content(f(&html)),
            EmailBlock::Wide(html) => EmailBlock::Wide(f(&html)),
            EmailBlock::FullWidthImage { src, alt, caption } => {
                EmailBlock::FullWidthImage { src: f(&src), alt: f(&alt), caption: caption.map(|caption| f(&caption)) }
            }
        }
    }
}

fn close_wrap(blocks: &mut Vec<EmailBlock>, buffer: &mut String, wrap: &mut Option<String>) {
    if let Some(float) = wrap.take() {
        let flow = std::mem::take(buffer);
        blocks.push(EmailBlock::Wide(format!(
            r#"{float}<div class="email-wrap" style="padding: 0 {WRAP_INSET_PX}px;">{flow}</div>{CLEAR}"#
        )));
    }
}

fn flush(blocks: &mut Vec<EmailBlock>, buffer: &mut String, wrap: &mut Option<String>) {
    close_wrap(blocks, buffer, wrap);
    if !buffer.is_empty() {
        blocks.push(EmailBlock::Content(std::mem::take(buffer)));
    }
}

pub fn render_email(document: &Document, theme: EmailTheme, urls: &Urls) -> Vec<EmailBlock> {
    let renderer = Renderer { theme, image_px: None, wood_src: urls.asset(WOOD_EMAIL_PATH), is_flush_top: false };
    let mut blocks = Vec::new();
    let mut buffer = String::new();
    let mut wrap: Option<String> = None;

    for block in document.blocks() {
        match block {
            Block::Figure { attrs, content } if attrs.width == ImageWidth::Full => {
                let Some((image, caption)) = figure_parts(content) else { continue };
                flush(&mut blocks, &mut buffer, &mut wrap);
                blocks.push(EmailBlock::FullWidthImage {
                    src: image.src.clone(),
                    alt: alt_text(caption),
                    caption: (!caption.is_empty()).then(|| only_with_name(caption, renderer.inlines(caption)).into_string()),
                });
            }
            Block::Figure { .. } | Block::Music { .. } | Block::Youtube { .. } => {
                flush(&mut blocks, &mut buffer, &mut wrap);
                let html = match block {
                    Block::Music { attrs } => renderer.music(attrs, urls),
                    Block::Youtube { attrs } => renderer.youtube(attrs),
                    _ => renderer.block(block, Context::Body),
                };
                blocks.push(EmailBlock::Wide(html.into_string()));
            }
            Block::Boxout { attrs, content } if attrs.width == BoxoutWidth::Wide => {
                flush(&mut blocks, &mut buffer, &mut wrap);
                blocks.push(EmailBlock::Wide(renderer.boxout(attrs, content).into_string()));
            }
            Block::Boxout { attrs, content } if attrs.width.is_side() => {
                flush(&mut blocks, &mut buffer, &mut wrap);
                wrap = Some(renderer.boxout(attrs, content).into_string());
            }
            Block::Boxout { attrs, content } => {
                close_wrap(&mut blocks, &mut buffer, &mut wrap);
                buffer.push_str(&renderer.boxout(attrs, content).into_string());
            }
            Block::Heading { .. } | Block::HorizontalRule {} if wrap.is_some() => {
                close_wrap(&mut blocks, &mut buffer, &mut wrap);
                buffer.push_str(&renderer.block(block, Context::Body).into_string());
            }
            _ => buffer.push_str(&renderer.block(block, Context::Body).into_string()),
        }
    }

    flush(&mut blocks, &mut buffer, &mut wrap);
    blocks
}

fn only_with_name(content: &[Inline], markup: Markup) -> Markup {
    if requires_name(content) {
        html! { (PreEscaped(personalize::IF_NAME)) (markup) (PreEscaped(personalize::END_IF_NAME)) }
    } else {
        markup
    }
}

pub fn caption_style(theme: &EmailTheme) -> String {
    format!(
        "margin: 8px 0 0 0; padding: 0; font-family: {}; font-size: {}px; line-height: 1.4; color: {};",
        theme.font_body,
        theme.text_px.saturating_sub(2),
        theme.text,
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Context {
    Body,
    Quote,
}

#[derive(Clone)]
struct Renderer {
    theme: EmailTheme,
    image_px: Option<u16>,
    wood_src: String,
    is_flush_top: bool,
}

impl Renderer {
    fn body_font(&self) -> String {
        format!("font-family: {}; font-size: {}px; line-height: 1.5;", self.theme.font_body, self.theme.text_px)
    }

    fn paragraph_style(&self, context: Context) -> String {
        match context {
            Context::Body => format!("margin: 0 0 16px 0; color: {}; {}", self.theme.text, self.body_font()),
            Context::Quote => "margin: 0 0 12px 0; color: inherit; font-family: inherit; font-size: inherit; line-height: inherit;".to_string(),
        }
    }

    fn heading_style(&self, level: u8) -> String {
        let align = if self.theme.layout == Layout::Centred { "center" } else { "left" };
        format!(
            "font-family: {}; font-size: {}px; font-weight: bold; text-align: {align}; margin: {}px 0 8px 0; line-height: 1.3;",
            self.theme.font_title,
            self.theme.heading_px(level.into()),
            self.top_margin(28),
        )
    }

    fn list_style(&self, list_style: &str) -> String {
        format!("padding: 0 0 0 24px; margin: 0 0 16px 0; list-style-type: {list_style};")
    }

    fn list_item_style(&self) -> String {
        format!("{} margin-bottom: 4px;", self.body_font())
    }

    fn blockquote_style(&self) -> String {
        format!("border-left: 3px solid {}; padding: 0 0 0 16px; margin: 0 0 16px 0; {}", self.theme.primary, self.body_font())
    }

    fn pullquote_style(&self) -> String {
        format!(
            "border-left: 4px solid {}; background-color: {}; padding: 20px 28px; margin: {}px 0 24px 0;",
            self.theme.primary, self.theme.surface_muted, self.top_margin(24),
        )
    }

    fn pullquote_body_style(&self) -> String {
        format!(
            "margin: 0; padding: 0; border: 0; font-family: {}; font-size: {}px; font-style: italic; line-height: 1.5;",
            self.theme.font_body,
            u16::from(self.theme.text_px) + 3,
        )
    }

    fn attribution_style(&self) -> String {
        format!(
            "margin: 16px 0 0 0; text-align: right; font-family: {}; font-size: {}px; line-height: 1.6; color: {};",
            self.theme.font_body,
            self.theme.text_px.saturating_sub(2),
            self.theme.muted,
        )
    }

    fn code_block_style(&self) -> String {
        format!(
            "font-family: monospace, monospace; font-size: 14px; background-color: {}; border: 1px solid {}; \
             padding: 16px; margin: 0 0 16px 0; display: block; white-space: pre-wrap; word-wrap: break-word;",
            self.theme.surface_muted, self.theme.border,
        )
    }

    fn hr_style(&self) -> String {
        format!("border: 0; border-top: 1px solid {}; margin: {}px 0 32px 0;", self.theme.border, self.top_margin(32))
    }

    fn link_style(&self) -> String {
        format!("color: {}; text-decoration: underline;", self.theme.link)
    }

    fn card_colors(&self, surface: Option<Color>) -> (Color, Color) {
        match surface {
            Some(surface) => (surface, WHITE),
            None => (self.theme.surface_muted, self.theme.text),
        }
    }

    fn card_style(&self, surface: Color) -> String {
        format!("margin: 0 0 16px 0; background-color: {surface}; border-collapse: separate;")
    }

    fn music_kind_style(&self, ink: Color) -> String {
        format!(
            "margin: 0 0 4px 0; font-family: {}; font-size: 11px; line-height: 1.4; letter-spacing: 0.14em; text-transform: uppercase; color: {ink};",
            self.theme.font_title,
        )
    }

    fn card_title_paragraph_style(&self) -> String {
        format!(
            "margin: 0 0 4px 0; font-family: {}; font-size: {}px; line-height: 1.25; font-weight: bold;",
            self.theme.font_title,
            u16::from(self.theme.text_px) + 2,
        )
    }

    fn card_title_style(&self, ink: Color) -> String {
        format!("color: {ink}; text-decoration: none;")
    }

    fn card_byline_style(&self, ink: Color) -> String {
        format!("margin: 0 0 8px 0; color: {ink};")
    }

    fn code_mark_style(&self) -> String {
        format!("font-family: monospace, monospace; font-size: 14px; background-color: {}; padding: 2px 4px;", self.theme.surface_muted)
    }

    fn flush_blocks(&self, blocks: &[Block]) -> Markup {
        let Some((first, rest)) = blocks.split_first() else { return html! {} };
        let flush = Renderer { is_flush_top: true, ..self.clone() };
        html! {
            (flush.block(first, Context::Body))
            (self.blocks(rest, Context::Body))
        }
    }

    fn top_margin(&self, px: u8) -> u8 {
        if self.is_flush_top { 0 } else { px }
    }

    fn blocks(&self, blocks: &[Block], context: Context) -> Markup {
        html! {
            @for block in blocks {
                (self.block(block, context))
            }
        }
    }

    fn block(&self, block: &Block, context: Context) -> Markup {
        let markup = self.block_markup(block, context);
        if block.requires_name() {
            html! { (PreEscaped(personalize::IF_NAME)) (markup) (PreEscaped(personalize::END_IF_NAME)) }
        } else {
            markup
        }
    }

    fn block_markup(&self, block: &Block, context: Context) -> Markup {
        match block {
            Block::Paragraph { content } | Block::Caption { content } | Block::Attribution { content } => {
                html! { p style=(self.paragraph_style(context)) { (self.inlines(content)) } }
            }
            Block::Heading { attrs, content } => {
                let style = self.heading_style(attrs.level);
                match attrs.level {
                    3 => html! { h3 style=(style) { (self.inlines(content)) } },
                    4 => html! { h4 style=(style) { (self.inlines(content)) } },
                    _ => html! { h2 style=(style) { (self.inlines(content)) } },
                }
            }
            Block::Blockquote { content, .. } => html! {
                blockquote style=(self.blockquote_style()) { (self.blocks(content, Context::Body)) }
            },
            Block::Pullquote { content } => {
                let (body, attribution) = quote_parts(content);
                html! {
                    div style=(self.pullquote_style()) {
                        blockquote style=(self.pullquote_body_style()) { (self.blocks(body, Context::Quote)) }
                        @if !attribution.is_empty() {
                            (only_with_name(attribution, html! { p style=(self.attribution_style()) { "\u{2014}" br; (self.inlines(attribution)) } }))
                        }
                    }
                }
            }
            Block::BulletList { content } => html! {
                ul style=(self.list_style("disc")) { (self.list_items(content, context)) }
            },
            Block::OrderedList { attrs, content } => {
                let start = (attrs.start != 1).then_some(attrs.start);
                html! {
                    ol start=[start] style=(self.list_style("decimal")) { (self.list_items(content, context)) }
                }
            }
            Block::CodeBlock { content, .. } => html! {
                pre style=(self.code_block_style()) { code { (code_text(content)) } }
            },
            Block::HorizontalRule {} => html! { hr style=(self.hr_style()); },
            Block::Image { attrs } => self.image(attrs, ""),
            Block::Figure { content, .. } => match figure_parts(content) {
                Some((image, caption)) => html! {
                    div style="margin: 0 0 16px 0; padding: 0;" {
                        (self.image(image, &alt_text(caption)))
                        @if !caption.is_empty() {
                            (only_with_name(caption, html! { p style=(caption_style(&self.theme)) { (self.inlines(caption)) } }))
                        }
                    }
                },
                None => html! {},
            },
            Block::Boxout { attrs, content } => self.boxout(attrs, content),
            Block::Music { .. } | Block::Youtube { .. } => html! {},
        }
    }

    fn boxout(&self, attrs: &BoxoutAttrs, content: &[Block]) -> Markup {
        let theme = self.theme;
        let frame_px = if attrs.style == BoxoutStyle::Trim { BOXOUT_TRIM_PX } else { 0 };
        let image_px = attrs.width.is_side().then_some(BOXOUT_SIDE_IMAGE_PX - 2 * frame_px as u16);
        let inner_theme = match attrs.style {
            BoxoutStyle::Bordered => theme,
            BoxoutStyle::Brand | BoxoutStyle::Trim => EmailTheme {
                text: theme.on_primary,
                link: theme.on_primary,
                primary: theme.on_primary,
                muted: theme.on_primary,
                border: theme.on_primary,
                surface_muted: theme.on_primary.mix(theme.primary, 0.15),
                ..theme
            },
        };
        let inner = Renderer { theme: inner_theme, image_px, wood_src: self.wood_src.clone(), is_flush_top: false };
        let (background, frame) = match attrs.style {
            BoxoutStyle::Bordered => (None, format!("border: 1px solid {};", theme.border)),
            BoxoutStyle::Brand | BoxoutStyle::Trim => (Some(theme.primary), format!("border: 0; background-color: {};", theme.primary)),
        };
        let panel = html! {
            table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" bgcolor=[background]
                style={ "border-collapse: separate; " (frame) } {
                tr {
                    td style={ "padding: " (BOXOUT_PADDING_PX) "px " (BOXOUT_PADDING_PX) "px 0 " (BOXOUT_PADDING_PX) "px; color: " (inner.theme.text) "; " (self.body_font()) } {
                        (inner.flush_blocks(content))
                    }
                }
            }
        };
        let panel = if attrs.style == BoxoutStyle::Trim { self.trim_frame(panel) } else { panel };
        let (align, gap) = match attrs.width {
            BoxoutWidth::Left => ("left", format!("padding: 4px {BOXOUT_GAP_PX}px 16px 0;")),
            BoxoutWidth::Right => ("right", format!("padding: 4px 0 16px {BOXOUT_GAP_PX}px;")),
            BoxoutWidth::Wide | BoxoutWidth::Text => return html! { div style="margin: 0 0 16px 0; padding: 0;" { (panel) } },
        };
        html! {
            table.email-boxout-side role="presentation" align=(align) width=(BOXOUT_SIDE_PX) cellpadding="0" cellspacing="0" border="0" {
                tr {
                    td.email-boxout-gap style=(gap) { (panel) }
                }
            }
        }
    }

    fn trim_frame(&self, panel: Markup) -> Markup {
        let theme = self.theme;
        let (color, image) = match theme.trim {
            Trim::Wood => (WOOD_COLOR, Some(self.wood_src.as_str())),
            Trim::Plain => (theme.primary, None),
            Trim::Color(color) => (color, None),
        };
        html! {
            table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" bgcolor=(color) background=[image]
                style={
                    "border-collapse: separate; background-color: " (color) ";"
                    @if let Some(image) = image { " background-image: url('" (image) "'); background-repeat: repeat;" }
                } {
                tr {
                    td style={ "padding: " (BOXOUT_TRIM_PX) "px;" } { (panel) }
                }
            }
        }
    }

    fn youtube(&self, video: &YoutubeAttrs) -> Markup {
        let href = video.watch_url();
        let (surface, ink) = self.card_colors(video.surface());
        let alt = if video.title.is_empty() { WATCH_LABEL } else { video.title.as_str() };
        html! {
            table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" bgcolor=(surface) style=(self.card_style(surface)) {
                tr {
                    td style={ "padding: " (CARD_PADDING_PX) "px; " (self.body_font()) } {
                        a href=(href) target="_blank" rel="noopener noreferrer" style="display: block; text-decoration: none;" {
                            img src=(video.thumbnail_url()) alt=(alt) width="560"
                                style="display: block; width: 100%; max-width: 100%; height: auto; margin: 0 0 10px 0; border: 0;";
                        }
                        @if !video.title.is_empty() {
                            p style=(self.card_title_paragraph_style()) {
                                a href=(href) target="_blank" rel="noopener noreferrer" style=(self.card_title_style(ink)) { (video.title) }
                            }
                        }
                        @if !video.channel.is_empty() {
                            p style=(self.card_byline_style(ink)) { (video.channel) }
                        }
                    }
                }
            }
        }
    }

    fn music(&self, music: &MusicAttrs, urls: &Urls) -> Markup {
        let href = music.primary().map(|(_, link)| link.url.as_str());
        let gap = if music.artwork.is_some() { MUSIC_GAP_PX } else { 0 };
        let (surface, ink) = self.card_colors(music.surface());
        html! {
            table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" bgcolor=(surface) style=(self.card_style(surface)) {
                tr {
                    td style={ "padding: " (MUSIC_PADDING_PX) "px;" } {
                        table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" {
                            tr {
                                @if let Some(artwork) = &music.artwork {
                                    td width=(MUSIC_ARTWORK_PX) style={ "width: " (MUSIC_ARTWORK_PX) "px; padding: 0; vertical-align: top;" } {
                                        a href=[href] target="_blank" rel="noopener noreferrer" style="text-decoration: none;" {
                                            img src=(artwork) alt="" width=(MUSIC_ARTWORK_PX) height=(MUSIC_ARTWORK_PX)
                                                style={ "display: block; width: " (MUSIC_ARTWORK_PX) "px; height: " (MUSIC_ARTWORK_PX) "px; max-width: none; border: 0;" };
                                        }
                                    }
                                }
                                td style={ "padding: 0 0 0 " (gap) "px; vertical-align: top; " (self.body_font()) } {
                                    (self.music_details(music, href, ink,))
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn music_details(&self, music: &MusicAttrs, href: Option<&str>, ink: Color) -> Markup {
        html! {
            @if music.kind == MusicKind::Album {
                p style=(self.music_kind_style(ink)) { "Album" }
            }
            @if !music.title.is_empty() {
                p style=(self.card_title_paragraph_style()) {
                    a href=[href] target="_blank" rel="noopener noreferrer" style=(self.card_title_style(ink)) { (music.title) }
                }
            }
            @if !music.artist.is_empty() {
                p style=(self.card_byline_style(ink)) { (music.artist) }
            }
            p style={ "margin: 0; font-size: 13px; line-height: " (MUSIC_ICON_PX) "px;" } {
                span style={ "display: inline-block; vertical-align: middle; margin: 0 8px 0 0; color: " (ink) ";" } { "Listen on" }
                @for (service, link) in music.links.iter() {
                    a href=(link.url) target="_blank" rel="noopener noreferrer" title=(listen_label(service))
                        style="display: inline-block; vertical-align: middle; margin: 0 6px 0 0; text-decoration: none;" {
                        (service.label())
                    }
                }
            }
        }
    }

    fn image(&self, attrs: &ImageAttrs, alt: &str) -> Markup {
        html! {
            img src=(attrs.src) alt=(alt) title=[attrs.title.as_deref()] width=[self.image_px] style="max-width: 100%; height: auto; display: block; margin: 0;";
        }
    }

    fn list_items(&self, items: &[ListItem], context: Context) -> Markup {
        html! {
            @for item in items {
                li style=(self.list_item_style()) {
                    @for (index, block) in item.content.iter().enumerate() {
                        @match (index, block) {
                            (0, Block::Paragraph { content }) => { (self.inlines(content)) }
                            _ => { (self.block(block, context)) }
                        }
                    }
                }
            }
        }
    }

    fn inlines(&self, content: &[Inline]) -> Markup {
        self.pieces(&inline_tree(content))
    }

    fn pieces(&self, pieces: &[InlinePiece]) -> Markup {
        html! {
            @for piece in pieces {
                @match piece {
                    InlinePiece::Text(text) => { (text) }
                    InlinePiece::Break => { br; }
                    InlinePiece::Name(fallback) => { (PreEscaped(personalize::NAME_START)) (fallback) (PreEscaped(personalize::NAME_END)) }
                    InlinePiece::Marked(mark, children) => { (self.marked(mark, children)) }
                }
            }
        }
    }

    fn marked(&self, mark: &Mark, children: &[InlinePiece]) -> Markup {
        let inner = self.pieces(children);
        match mark {
            Mark::Link { attrs } => html! {
                a href=(attrs.href) target="_blank" rel="noopener noreferrer" style=(self.link_style()) { (inner) }
            },
            Mark::Bold {} => html! { strong { (inner) } },
            Mark::Italic {} => html! { em { (inner) } },
            Mark::Underline {} => html! { u { (inner) } },
            Mark::Strike {} => html! { s { (inner) } },
            Mark::Code {} => html! { code style=(self.code_mark_style()) { (inner) } },
        }
    }
}
