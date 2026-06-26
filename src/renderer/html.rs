use serde_json::Value;

/// Render a ProseMirror JSON document to an HTML string.
pub fn render(content: &Value) -> String {
    render_node(content)
}

fn render_node(node: &Value) -> String {
    match node["type"].as_str().unwrap_or("") {
        "doc" => render_children(node),

        "paragraph" => format!("<p>{}</p>", render_children(node)),

        "text" => {
            let text = escape_html(node["text"].as_str().unwrap_or(""));
            match node["marks"].as_array() {
                Some(marks) => apply_marks(text, marks),
                None => text,
            }
        }

        "heading" => {
            let level = node["attrs"]["level"].as_u64().unwrap_or(1).clamp(1, 6);
            format!("<h{level}>{}</h{level}>", render_children(node))
        }

        "bulletList" => format!("<ul>{}</ul>", render_children(node)),
        "orderedList" => format!("<ol>{}</ol>", render_children(node)),
        "listItem" => format!("<li>{}</li>", render_children(node)),

        "blockquote" => format!("<blockquote>{}</blockquote>", render_children(node)),

        "codeBlock" => {
            let lang = node["attrs"]["language"].as_str().unwrap_or("");
            let class = if lang.is_empty() {
                String::new()
            } else {
                format!(" class=\"language-{}\"", escape_html(lang))
            };
            format!("<pre><code{}>{}</code></pre>", class, render_children(node))
        }

        "horizontalRule" => "<hr>".to_string(),
        "hardBreak" => "<br>".to_string(),

        "image" => {
            let src = escape_html(node["attrs"]["src"].as_str().unwrap_or(""));
            let alt = escape_html(node["attrs"]["alt"].as_str().unwrap_or(""));
            let title = node["attrs"]["title"].as_str();
            match title {
                Some(t) => format!("<img src=\"{}\" alt=\"{}\" title=\"{}\">", src, alt, escape_html(t)),
                None => format!("<img src=\"{}\" alt=\"{}\">", src, alt),
            }
        }

        // Unknown node types — render children so content isn't silently dropped
        _ => render_children(node),
    }
}

fn render_children(node: &Value) -> String {
    node["content"]
        .as_array()
        .map(|children| children.iter().map(render_node).collect::<String>())
        .unwrap_or_default()
}

/// Apply marks outermost-first (first in array = outermost wrapper).
fn apply_marks(content: String, marks: &[Value]) -> String {
    marks.iter().fold(content, |acc, mark| {
        match mark["type"].as_str().unwrap_or("") {
            "bold" => format!("<strong>{acc}</strong>"),
            "italic" => format!("<em>{acc}</em>"),
            "underline" => format!("<u>{acc}</u>"),
            "strike" => format!("<s>{acc}</s>"),
            "code" => format!("<code>{acc}</code>"),
            "link" => {
                let href = escape_html(mark["attrs"]["href"].as_str().unwrap_or("#"));
                let target = mark["attrs"]["target"].as_str().unwrap_or("_blank");
                format!("<a href=\"{href}\" target=\"{target}\">{acc}</a>")
            }
            _ => acc,
        }
    })
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
