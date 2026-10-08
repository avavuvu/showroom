use serde_json::Value;

use crate::theme::{EmailTheme, Layout};

pub enum EmailBlock {
    Content(String),
    FullWidthImage { src: String, alt: String },
}

pub fn render_email(content: &Value, theme: EmailTheme) -> Vec<EmailBlock> {
    Renderer { theme }.render_blocks(content)
}

struct Renderer {
    theme: EmailTheme,
}

impl Renderer {
    fn render_blocks(&self, doc: &Value) -> Vec<EmailBlock> {
        let mut blocks = Vec::new();
        let mut buffer = String::new();



        if let Some(children) = doc["content"].as_array() {


            for child in children {
                let is_full_width_image = child["type"].as_str() == Some("image")
                    && child["attrs"]["width"].as_str() == Some("full");

                if is_full_width_image {
                    if !buffer.is_empty() {
                        blocks.push(EmailBlock::Content(std::mem::take(&mut buffer)));
                    }
                    blocks.push(EmailBlock::FullWidthImage {
                        src: escape_html(child["attrs"]["src"].as_str().unwrap_or("")),
                        alt: escape_html(child["attrs"]["alt"].as_str().unwrap_or("")),
                    });
                } else {
                    buffer.push_str(&self.render_node(child));
                }
            }
        }

        if !buffer.is_empty() {
            blocks.push(EmailBlock::Content(buffer));
        }

        blocks
    }

    fn s(&self, style: String) -> String {
        format!(" style=\"{}\"", style.replace('"', "&quot;"))
    }

    fn paragraph_style(&self) -> String {
        let font_body = self.theme.font_body;
        let size = self.theme.text_px;
        self.s(format!(
            "margin: 0 0 16px 0; font-family: {font_body}; font-size: {size}px; line-height: 1.5;"
        ))
    }

    fn heading_style(&self, level: u64) -> String {
        let font = if level <= 3 { self.theme.font_title } else { self.theme.font_body };
        let size = self.theme.heading_px(level);
        let align = if self.theme.layout == Layout::Centred { "center" } else { "left" };
        self.s(format!(
            "font-family: {font}; font-size: {size}px; font-weight: bold; text-align: {align}; \
             margin: 28px 0 8px 0; line-height: 1.3;"
        ))
    }

    fn list_style(&self, kind: &str) -> String {
        let list_style = if kind == "bullet" { "disc" } else { "decimal" };
        self.s(format!(
            "padding: 0 0 0 24px; margin: 0 0 16px 0; list-style-type: {list_style};"
        ))
    }

    fn list_item_style(&self) -> String {
        let font_body = self.theme.font_body;
        let size = self.theme.text_px;
        self.s(format!(
            "font-family: {font_body}; font-size: {size}px; line-height: 1.5; margin-bottom: 4px;"
        ))
    }

    fn blockquote_style(&self) -> String {
        let font_body = self.theme.font_body;
        let size = self.theme.text_px;
        self.s(format!(
            "border-left: 3px solid {}; padding: 0 0 0 16px; margin: 0 0 16px 0; \
             font-family: {font_body}; font-size: {size}px; line-height: 1.5;",
            self.theme.primary
        ))
    }

    fn code_block_style(&self) -> String {
        self.s(format!(
            "font-family: monospace, monospace; font-size: 14px; \
             background-color: {}; border: 1px solid {}; padding: 16px; margin: 0 0 16px 0; \
             display: block; white-space: pre-wrap; word-wrap: break-word;",
            self.theme.surface_muted, self.theme.border,
        ))
    }

    fn hr_style(&self) -> String {
        self.s(format!(
            "border: 0; border-top: 1px solid {}; margin: 32px 0;",
            self.theme.surface_muted
        ))
    }

    fn image_style(&self) -> String {
        // display: block removes the bottom gap email clients add under images
        self.s("max-width: 100%; height: auto; display: block;".to_string())
    }

    fn link_style(&self) -> String {
        self.s(format!("color: {}; text-decoration: underline;", self.theme.link))
    }

    fn code_mark_style(&self) -> String {
        self.s(format!(
            "font-family: monospace, monospace; font-size: 14px; \
             background-color: {}; padding: 2px 4px;",
            self.theme.surface_muted
        ))
    }

    fn render_node(&self, node: &Value) -> String {
        match node["type"].as_str().unwrap_or("") {
            "paragraph" => {
                format!("<p{}>{}</p>", self.paragraph_style(), self.render_children(node))
            }

            "text" => {
                let text = escape_html(node["text"].as_str().unwrap_or(""));
                match node["marks"].as_array() {
                    Some(marks) => self.apply_marks(text, marks),
                    None => text,
                }
            }

            "heading" => {
                let level = node["attrs"]["level"].as_u64().unwrap_or(1).clamp(1, 6);
                let style = self.heading_style(level);
                format!("<h{level}{style}>{}</h{level}>", self.render_children(node))
            }

            "bulletList" => {
                format!("<ul{}>{}</ul>", self.list_style("bullet"), self.render_children(node))
            }

            "orderedList" => {
                format!("<ol{}>{}</ol>", self.list_style("ordered"), self.render_children(node))
            }

            "listItem" => {
                format!("<li{}>{}</li>", self.list_item_style(), self.render_children(node))
            }

            "blockquote" => {
                format!("<blockquote{}>{}</blockquote>", self.blockquote_style(), self.render_children(node))
            }

            "codeBlock" => {
                let lang = node["attrs"]["language"].as_str().unwrap_or("");
                let class = if lang.is_empty() {
                    String::new()
                } else {
                    format!(" class=\"language-{}\"", escape_html(lang))
                };
                format!("<pre{}><code{class}>{}</code></pre>", self.code_block_style(), self.render_children(node))
            }

            "horizontalRule" => format!("<hr{}>", self.hr_style()),

            "hardBreak" => "<br>".to_string(),

            "image" => {
                let src   = escape_html(node["attrs"]["src"].as_str().unwrap_or(""));
                let alt   = escape_html(node["attrs"]["alt"].as_str().unwrap_or(""));
                let style = self.image_style();
                match node["attrs"]["title"].as_str() {
                    Some(t) => format!("<img src=\"{src}\" alt=\"{alt}\" title=\"{}\"{style}>", escape_html(t)),
                    None    => format!("<img src=\"{src}\" alt=\"{alt}\"{style}>"),
                }
            }

            _ => self.render_children(node),
        }
    }

    fn render_children(&self, node: &Value) -> String {
        node["content"]
            .as_array()
            .map(|children| children.iter().map(|n| self.render_node(n)).collect::<String>())
            .unwrap_or_default()
    }

    fn apply_marks(&self, content: String, marks: &[Value]) -> String {
        marks.iter().fold(content, |acc, mark| {
            match mark["type"].as_str().unwrap_or("") {
                "bold"      => format!("<strong>{acc}</strong>"),
                "italic"    => format!("<em>{acc}</em>"),
                "underline" => format!("<u>{acc}</u>"),
                "strike"    => format!("<s>{acc}</s>"),
                "code"      => format!("<code{}>{acc}</code>", self.code_mark_style()),
                "link" => {
                    let href   = escape_html(mark["attrs"]["href"].as_str().unwrap_or("#"));
                    let target = mark["attrs"]["target"].as_str().unwrap_or("_blank");
                    format!("<a href=\"{href}\" target=\"{target}\"{}>{acc}</a>", self.link_style())
                }
                _ => acc,
            }
        })
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
