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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{EmailBlock, render_email};
    use crate::document::Document;
    use crate::theme::EmailTheme;

    fn blocks(content: serde_json::Value) -> Vec<EmailBlock> {
        let document = Document::from_value(&json!({ "type": "doc", "content": content })).expect("valid document");
        render_email(&document, EmailTheme::default())
    }

    fn single(content: serde_json::Value) -> String {
        let mut blocks = blocks(content);
        assert_eq!(blocks.len(), 1, "expected one block");
        match blocks.remove(0) {
            EmailBlock::Content(html) => html,
            EmailBlock::FullWidthImage { .. } => panic!("expected content"),
        }
    }

    fn paragraph(value: &str) -> serde_json::Value {
        json!({ "type": "paragraph", "content": [{ "type": "text", "text": value }] })
    }

    fn figure(src: &str, width: &str, caption: serde_json::Value) -> serde_json::Value {
        json!({ "type": "figure", "attrs": { "width": width }, "content": [
            { "type": "image", "attrs": { "src": src } },
            { "type": "caption", "content": caption }
        ] })
    }

    #[test]
    fn empty_document_has_no_blocks() {
        assert!(blocks(json!([])).is_empty());
        assert!(blocks(json!([{ "type": "paragraph" }])).is_empty());
    }

    #[test]
    fn paragraphs_have_inline_styles_and_escaped_text() {
        let html = single(json!([paragraph("1 < 2 & \"3\"")]));
        assert!(html.starts_with("<p style=\"margin: 0 0 16px 0; font-family: "), "{html}");
        assert!(html.ends_with(">1 &lt; 2 &amp; &quot;3&quot;</p>"), "{html}");
    }

    #[test]
    fn style_values_cannot_break_out_of_the_attribute() {
        let html = single(json!([paragraph("x")]));
        let style_start = html.find("style=\"").expect("style attribute") + 7;
        let style_end = style_start + html[style_start..].find('"').expect("closing quote");
        assert_eq!(&html[style_end..], "\">x</p>", "{html}");
    }

    #[test]
    fn full_width_figures_split_the_content() {
        let result = blocks(json!([
            paragraph("before"),
            figure("https://res.cloudinary.com/demo/a.jpg?x=1&y=2", "full", json!([
                { "type": "text", "text": "A " },
                { "type": "text", "text": "cat", "marks": [{ "type": "italic" }] }
            ])),
            paragraph("after"),
            figure("https://res.cloudinary.com/demo/b.jpg", "normal", json!([{ "type": "text", "text": "B <i>" }])),
            { "type": "image", "attrs": { "src": "https://res.cloudinary.com/demo/c.jpg", "alt": "old alt" } }
        ]));
        assert_eq!(result.len(), 3);
        assert!(matches!(&result[0], EmailBlock::Content(html) if html.contains(">before</p>")));
        assert!(matches!(
            &result[1],
            EmailBlock::FullWidthImage { src, caption } if src == "https://res.cloudinary.com/demo/a.jpg?x=1&y=2" && caption.as_deref() == Some("A <em>cat</em>")
        ));
        let EmailBlock::Content(html) = &result[2] else { panic!("expected content") };
        assert!(html.contains(">after</p>"), "{html}");
        assert!(html.contains("<img src=\"https://res.cloudinary.com/demo/b.jpg\" alt=\"\" style=\""), "{html}");

        assert!(html.contains(">B &lt;i&gt;</p></div>"), "{html}");
        assert!(html.contains("<img src=\"https://res.cloudinary.com/demo/c.jpg\" alt=\"\" style=\""), "{html}");
        assert!(!html.contains("old alt"), "{html}");
    }

    #[test]
    fn full_width_figures_without_a_caption() {
        let result = blocks(json!([figure("https://res.cloudinary.com/demo/a.jpg", "full", json!([]))]));
        assert!(matches!(&result[0], EmailBlock::FullWidthImage { caption: None, .. }));
    }

    #[test]
    fn headings_use_h2_to_h4() {
        let html = single(json!([
            { "type": "heading", "attrs": { "level": 1 }, "content": [{ "type": "text", "text": "A" }] },
            { "type": "heading", "attrs": { "level": 3 }, "content": [{ "type": "text", "text": "B" }] },
            { "type": "heading", "attrs": { "level": 4 }, "content": [{ "type": "text", "text": "C" }] }
        ]));
        assert!(html.starts_with("<h2 style=\""), "{html}");
        assert!(html.contains(">A</h2><h3 style=\""), "{html}");
        assert!(html.contains(">B</h3><h4 style=\""), "{html}");
        assert!(html.ends_with(">C</h4>"), "{html}");
    }

    #[test]
    fn links_are_styled_and_safe() {
        let html = single(json!([
            { "type": "paragraph", "content": [
                { "type": "text", "text": "ok", "marks": [{ "type": "link", "attrs": { "href": "https://example.com", "target": "\" onclick=\"x" } }] },
                { "type": "text", "text": "bad", "marks": [{ "type": "link", "attrs": { "href": "javascript:alert(1)" } }] }
            ] }
        ]));
        assert!(html.contains("<a href=\"https://example.com\" target=\"_blank\" rel=\"noopener noreferrer\" style=\"color: "), "{html}");
        assert!(html.contains(">ok</a>bad</p>"), "{html}");
        assert!(!html.contains("onclick"), "{html}");
        assert!(!html.contains("javascript"), "{html}");
    }

    #[test]
    fn attributed_quotes_show_the_source() {
        let standard = single(json!([{ "type": "blockquote", "content": [paragraph("q")] }]));
        let attributed = single(json!([{ "type": "pullquote", "content": [
            paragraph("q"),
            { "type": "attribution", "content": [{ "type": "text", "text": "Ava" }] }
        ] }]));
        assert!(standard.starts_with("<blockquote style=\"border-left: 3px solid "), "{standard}");
        assert!(attributed.starts_with("<div style=\"border-left: 4px solid "), "{attributed}");
        assert!(attributed.contains("font-style: italic"), "{attributed}");
        assert!(attributed.contains("font-size: inherit"), "{attributed}");
        assert!(attributed.contains("text-align: right"), "{attributed}");
        assert!(attributed.ends_with(">\u{2014}<br>Ava</p></div>"), "{attributed}");
    }

    #[test]
    fn list_items_inline_their_first_paragraph() {
        let html = single(json!([
            { "type": "orderedList", "attrs": { "start": 2 }, "content": [
                { "type": "listItem", "content": [paragraph("a"), { "type": "bulletList", "content": [
                    { "type": "listItem", "content": [paragraph("b")] }
                ] }] }
            ] }
        ]));
        assert!(html.starts_with("<ol start=\"2\" style=\""), "{html}");
        assert!(html.contains(">a<ul style=\""), "{html}");
        assert!(html.contains(">b</li></ul></li></ol>"), "{html}");
        assert!(!html.contains("<p"), "{html}");
    }

    #[test]
    fn code_is_escaped() {
        let html = single(json!([
            { "type": "codeBlock", "attrs": { "language": "html" }, "content": [{ "type": "text", "text": "<b>x</b>" }] }
        ]));
        assert!(html.ends_with("><code>&lt;b&gt;x&lt;/b&gt;</code></pre>"), "{html}");
    }
}
