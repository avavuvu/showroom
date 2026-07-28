use serde_json::Value;

pub fn render(content: &Value) -> String {
    Renderer.render_node(content)
}

struct Renderer;

impl Renderer {
    fn render_node(&self, node: &Value) -> String {
        match node["type"].as_str().unwrap_or("") {
            "doc" => self.render_children(node),

            "paragraph" => format!("<p>{}</p>", self.render_children(node)),

            "text" => {
                let text = escape_html(node["text"].as_str().unwrap_or(""));
                match node["marks"].as_array() {
                    Some(marks) => self.apply_marks(text, marks),
                    None => text,
                }
            }

            "heading" => {
                let level = node["attrs"]["level"].as_u64().unwrap_or(1).clamp(1, 6);
                format!("<h{level}>{}</h{level}>", self.render_children(node))
            }

            "bulletList" => format!("<ul>{}</ul>", self.render_children(node)),

            "orderedList" => format!("<ol>{}</ol>", self.render_children(node)),

            "listItem" => format!("<li>{}</li>", self.render_children(node)),

            "blockquote" => format!("<blockquote>{}</blockquote>", self.render_children(node)),

            "codeBlock" => {
                let lang = node["attrs"]["language"].as_str().unwrap_or("");
                let class = if lang.is_empty() {
                    String::new()
                } else {
                    format!(" class=\"language-{}\"", escape_html(lang))
                };
                format!("<pre><code{class}>{}</code></pre>", self.render_children(node))
            }

            "horizontalRule" => "<hr>".to_string(),

            "hardBreak" => "<br>".to_string(),

            "image" => {
                let src   = escape_html(node["attrs"]["src"].as_str().unwrap_or(""));
                let alt   = escape_html(node["attrs"]["alt"].as_str().unwrap_or(""));
                let width = escape_html(node["attrs"]["width"].as_str().unwrap_or("normal"));
                match node["attrs"]["title"].as_str() {
                    Some(t) => format!("<img src=\"{src}\" alt=\"{alt}\" title=\"{}\" data-width=\"{width}\">", escape_html(t)),
                    None    => format!("<img src=\"{src}\" alt=\"{alt}\" data-width=\"{width}\">"),
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
                "code"      => format!("<code>{acc}</code>"),
                "link" => {
                    let href   = escape_html(mark["attrs"]["href"].as_str().unwrap_or("#"));
                    let target = mark["attrs"]["target"].as_str().unwrap_or("_blank");
                    format!("<a href=\"{href}\" target=\"{target}\">{acc}</a>")
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
