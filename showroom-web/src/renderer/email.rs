use boutique::html;
use maud::Markup;

use crate::document::{
    Block, Document, ImageAttrs, ImageWidth, Inline, InlinePiece, ListItem, Mark, code_text, figure_parts, inline_tree, quote_parts,
};
use crate::theme::{EmailTheme, Layout};

pub enum EmailBlock {
    Content(String),
    FullWidthImage { src: String, caption: Option<String> },
}

pub fn render_email(document: &Document, theme: EmailTheme) -> Vec<EmailBlock> {
    let renderer = Renderer { theme };
    let mut blocks = Vec::new();
    let mut buffer = String::new();

    for block in document.blocks() {
        match block {
            Block::Figure { attrs, content } if attrs.width == ImageWidth::Full => {
                let Some((image, caption)) = figure_parts(content) else { continue };
                if !buffer.is_empty() {
                    blocks.push(EmailBlock::Content(std::mem::take(&mut buffer)));
                }
                blocks.push(EmailBlock::FullWidthImage {
                    src: image.src.clone(),
                    caption: (!caption.is_empty()).then(|| renderer.inlines(caption).into_string()),
                });
            }
            _ => buffer.push_str(&renderer.block(block, Context::Body).into_string()),
        }
    }

    if !buffer.is_empty() {
        blocks.push(EmailBlock::Content(buffer));
    }

    blocks
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

struct Renderer {
    theme: EmailTheme,
}

impl Renderer {
    fn body_font(&self) -> String {
        format!("font-family: {}; font-size: {}px; line-height: 1.5;", self.theme.font_body, self.theme.text_px)
    }

    fn paragraph_style(&self, context: Context) -> String {
        match context {
            Context::Body => format!("margin: 0 0 16px 0; {}", self.body_font()),
            Context::Quote => "margin: 0 0 12px 0; font-family: inherit; font-size: inherit; line-height: inherit;".to_string(),
        }
    }

    fn heading_style(&self, level: u8) -> String {
        let align = if self.theme.layout == Layout::Centred { "center" } else { "left" };
        format!(
            "font-family: {}; font-size: {}px; font-weight: bold; text-align: {align}; margin: 28px 0 8px 0; line-height: 1.3;",
            self.theme.font_title,
            self.theme.heading_px(level.into()),
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
            "border-left: 4px solid {}; background-color: {}; padding: 20px 28px; margin: 24px 0;",
            self.theme.primary, self.theme.surface_muted,
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
        format!("border: 0; border-top: 1px solid {}; margin: 32px 0;", self.theme.border)
    }

    fn link_style(&self) -> String {
        format!("color: {}; text-decoration: underline;", self.theme.link)
    }

    fn code_mark_style(&self) -> String {
        format!("font-family: monospace, monospace; font-size: 14px; background-color: {}; padding: 2px 4px;", self.theme.surface_muted)
    }

    fn blocks(&self, blocks: &[Block], context: Context) -> Markup {
        html! {
            @for block in blocks {
                (self.block(block, context))
            }
        }
    }

    fn block(&self, block: &Block, context: Context) -> Markup {
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
                            p style=(self.attribution_style()) { "\u{2014}" br; (self.inlines(attribution)) }
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
            Block::Image { attrs } => self.image(attrs),
            Block::Figure { content, .. } => match figure_parts(content) {
                Some((image, caption)) => html! {
                    div style="margin: 0 0 16px 0; padding: 0;" {
                        (self.image(image))
                        @if !caption.is_empty() {
                            p style=(caption_style(&self.theme)) { (self.inlines(caption)) }
                        }
                    }
                },
                None => html! {},
            },
        }
    }

    fn image(&self, attrs: &ImageAttrs) -> Markup {
        html! {
            img src=(attrs.src) alt="" title=[attrs.title.as_deref()] style="max-width: 100%; height: auto; display: block; margin: 0;";
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
