use serde_json::Value;

pub fn render(content: &Value) -> String {
    render_node(content, 0).trim_end().to_string()
}

fn render_node(node: &Value, list_depth: usize) -> String {
    match node["type"].as_str().unwrap_or("") {
        "doc" => render_children(node, list_depth),

        "paragraph" => {
            let inner = render_children(node, list_depth);
            if inner.is_empty() {
                "\n".to_string()
            } else {
                format!("{inner}\n\n")
            }
        }

        "text" => node["text"].as_str().unwrap_or("").to_string(),

        "heading" => {
            format!("{}\n\n", render_children(node, list_depth))
        }

        "bulletList" | "orderedList" => {
            format!("{}\n", render_children(node, list_depth + 1))
        }

        "listItem" => {
            let indent = "  ".repeat(list_depth.saturating_sub(1));
            let content = render_children(node, list_depth).trim_end().to_string();
            format!("{indent}- {content}\n")
        }

        "blockquote" => {
            let inner = render_children(node, list_depth);
            inner
                .lines()
                .map(|line| format!("> {line}"))
                .collect::<Vec<_>>()
                .join("\n")
                + "\n\n"
        }

        "codeBlock" => {
            format!("{}\n\n", render_children(node, list_depth))
        }

        "horizontalRule" => "---\n\n".to_string(),

        "hardBreak" => "\n".to_string(),

        "image" => {
            let alt = node["attrs"]["alt"].as_str().unwrap_or("");
            let src = node["attrs"]["src"].as_str().unwrap_or("");
            if alt.is_empty() {
                format!("{src}\n")
            } else {
                format!("{alt} ({src})\n")
            }
        }

        _ => render_children(node, list_depth),
    }
}

fn render_children(node: &Value, list_depth: usize) -> String {
    node["content"]
        .as_array()
        .map(|children| {
            children
                .iter()
                .map(|n| render_node(n, list_depth))
                .collect::<String>()
        })
        .unwrap_or_default()
}
