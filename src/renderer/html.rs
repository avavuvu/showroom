use serde_json::Value;

#[derive(Clone, Debug)]
pub struct ThemeVariables {
    pub font_body: String,
    pub font_title: String,
    pub color_text: String,
    pub color_primary: String,
    pub color_surface: String,
    pub color_surface_subtle: String,
    pub color_muted: String,
}

impl Default for ThemeVariables {
    fn default() -> Self {
        Self {
            font_body:                 r#""Times", "Times New Roman", serif"#.into(),
            font_title:                r#""Playfair Display", "Georgia", serif"#.into(),
            color_text:                "#000000".into(),
            color_primary:             "#92ca3a".into(),
            color_surface:             "#ffffff".into(),
            color_surface_subtle:      "#cccccc".into(),
            color_muted:               "#444444".into(),
        }
    }
}

pub fn render(content: &Value) -> String {
    Renderer { theme: None }.render(content)
}

pub fn render_email(content: &Value, theme: ThemeVariables) -> String {
    Renderer { theme: Some(theme) }.render(content)
}

struct Renderer {
    theme: Option<ThemeVariables>,
}

impl Renderer {
    fn render(&self, content: &Value) -> String {
        self.render_node(content)
    }

    fn is_email(&self) -> bool {
        self.theme.is_some()
    }

    fn v(&self) -> Option<&ThemeVariables> {
        self.theme.as_ref()
    }

    fn s(&self, style: String) -> String {
        if style.is_empty() {
            String::new()
        } else {
            format!(" style=\"{}\"", style.replace('"', "&quot;"))
        }
    }

    fn paragraph_style(&self) -> String {
        let Some(v) = self.v() else { return String::new() };
        self.s(format!(
            "margin: 0 0 16px 0; font-family: {}; font-size: 16px; line-height: 1.5;",
            v.font_body
        ))
    }

    fn heading_style(&self, level: u64) -> String {
        let Some(v) = self.v() else { return String::new() };
        let font = if level <= 3 { &v.font_title } else { &v.font_body };
        let size = match level {
            1 => 22, 2 => 19, 3 => 18, _ => 16,
        };
        self.s(format!(
            "font-family: {font}; font-size: {size}px; font-weight: bold; \
             margin: 0 0 8px 0; line-height: 1.3;"
        ))
    }

    fn list_style(&self, kind: &str) -> String {
        let Some(_) = self.v() else { return String::new() };
        let list_style = if kind == "bullet" { "disc" } else { "decimal" };
        // Outlook may ignore padding-left on lists; setting both padding and margin helps
        self.s(format!(
            "padding: 0 0 0 24px; margin: 0 0 16px 0; list-style-type: {list_style};"
        ))
    }

    fn list_item_style(&self) -> String {
        let Some(v) = self.v() else { return String::new() };
        self.s(format!(
            "font-family: {}; font-size: 16px; line-height: 1.5; margin-bottom: 4px;",
            v.font_body
        ))
    }

    fn blockquote_style(&self) -> String {
        let Some(v) = self.v() else { return String::new() };
        self.s(format!(
            "border-left: 3px solid {}; padding: 0 0 0 16px; margin: 0 0 16px 0; \
             font-family: {}; font-size: 16px; line-height: 1.5;",
            v.color_primary, v.font_body
        ))
    }

    fn code_block_style(&self) -> String {
        let Some(v) = self.v() else { return String::new() };
        self.s(format!(
            "font-family: monospace, monospace; font-size: 14px; \
             background-color: {}; padding: 16px; margin: 0 0 16px 0; \
             display: block; white-space: pre-wrap; word-wrap: break-word;",
            v.color_surface_subtle
        ))
    }

    fn hr_style(&self) -> String {
        let Some(v) = self.v() else { return String::new() };
        self.s(format!(
            "border: 0; border-top: 1px solid {}; margin: 32px 0;",
            v.color_surface_subtle
        ))
    }

    fn image_style(&self) -> String {
        // display: block removes the bottom gap email clients add under images
        if self.is_email() {
            self.s("max-width: 100%; height: auto; display: block;".to_string())
        } else {
            String::new()
        }
    }

    fn link_style(&self) -> String {
        let Some(v) = self.v() else { return String::new() };
        self.s(format!("color: {}; text-decoration: underline;", v.color_primary))
    }

    fn code_mark_style(&self) -> String {
        let Some(v) = self.v() else { return String::new() };
        // border-radius is stripped by Outlook but improves other clients
        self.s(format!(
            "font-family: monospace, monospace; font-size: 14px; \
             background-color: {}; padding: 2px 4px;",
            v.color_surface_subtle
        ))
    }

    fn render_node(&self, node: &Value) -> String {
        match node["type"].as_str().unwrap_or("") {
            "doc" => {
                let children = self.render_children(node);
                // In email mode wrap in a div that establishes the base font,
                // since font-family inheritance is unreliable across email clients
                if let Some(v) = self.v() {
                    format!(
                        "<div style=\"font-family: {}; font-size: 16px; line-height: 1.5;\">{children}</div>",
                        v.font_body
                    )
                } else {
                    children
                }
            }

            "paragraph" => {
                let style = self.paragraph_style();
                format!("<p{style}>{}</p>", self.render_children(node))
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
                let style = self.list_style("bullet");
                format!("<ul{style}>{}</ul>", self.render_children(node))
            }

            "orderedList" => {
                let style = self.list_style("ordered");
                format!("<ol{style}>{}</ol>", self.render_children(node))
            }

            "listItem" => {
                let style = self.list_item_style();
                format!("<li{style}>{}</li>", self.render_children(node))
            }

            "blockquote" => {
                let style = self.blockquote_style();
                format!("<blockquote{style}>{}</blockquote>", self.render_children(node))
            }

            "codeBlock" => {
                let lang = node["attrs"]["language"].as_str().unwrap_or("");
                let class = if lang.is_empty() {
                    String::new()
                } else {
                    format!(" class=\"language-{}\"", escape_html(lang))
                };
                let style = self.code_block_style();
                format!("<pre{style}><code{class}>{}</code></pre>", self.render_children(node))
            }

            "horizontalRule" => {
                let style = self.hr_style();
                format!("<hr{style}>")
            }

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

            // Unknown nodes — render children so content is never silently dropped
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
                "code"      => {
                    let style = self.code_mark_style();
                    format!("<code{style}>{acc}</code>")
                }
                "link" => {
                    let href   = escape_html(mark["attrs"]["href"].as_str().unwrap_or("#"));
                    let target = mark["attrs"]["target"].as_str().unwrap_or("_blank");
                    let style  = self.link_style();
                    format!("<a href=\"{href}\" target=\"{target}\"{style}>{acc}</a>")
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
