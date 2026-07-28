use serde_json::Value;

pub fn render(content: &Value) -> String {
    render_node(content, false, 0).trim_end().to_string()
}

fn render_node(node: &Value, ordered: bool, depth: usize) -> String {
    match node["type"].as_str().unwrap_or("") {
        "doc" => render_children(node, false, 0),

        "paragraph" => {
            let inner = render_children(node, ordered, depth);
            if inner.is_empty() {
                "\n".to_string()
            } else {
                format!("{inner}\n\n")
            }
        }

        "text" => {
            let text = node["text"].as_str().unwrap_or("").to_string();
            match node["marks"].as_array() {
                Some(marks) => apply_marks(text, marks),
                None => text,
            }
        }

        "heading" => {
            let level = node["attrs"]["level"].as_u64().unwrap_or(1).clamp(1, 6) as usize;
            let hashes = "#".repeat(level);
            format!("{hashes} {}\n\n", render_children(node, false, 0))
        }

        "bulletList"  => format!("{}\n", render_children(node, false, depth + 1)),
        "orderedList" => format!("{}\n", render_children(node, true,  depth + 1)),

        "listItem" => {
            let indent = "  ".repeat(depth.saturating_sub(1));
            let marker = if ordered { "1." } else { "-" };
            let content = render_children(node, ordered, depth).trim_end().to_string();
            format!("{indent}{marker} {content}\n")
        }

        "blockquote" => {
            let inner = render_children(node, ordered, depth);
            inner
                .lines()
                .map(|l| format!("> {l}"))
                .collect::<Vec<_>>()
                .join("\n")
                + "\n\n"
        }

        "codeBlock" => {
            let lang = node["attrs"]["language"].as_str().unwrap_or("");
            let inner = render_children(node, false, 0);
            format!("```{lang}\n{inner}\n```\n\n")
        }

        "horizontalRule" => "---\n\n".to_string(),

        "hardBreak" => "  \n".to_string(),

        "image" => {
            let alt = node["attrs"]["alt"].as_str().unwrap_or("");
            let src = node["attrs"]["src"].as_str().unwrap_or("");
            format!("![{alt}]({src})\n\n")
        }

        _ => render_children(node, ordered, depth),
    }
}

fn render_children(node: &Value, ordered: bool, depth: usize) -> String {
    node["content"]
        .as_array()
        .map(|children| {
            children
                .iter()
                .map(|n| render_node(n, ordered, depth))
                .collect()
        })
        .unwrap_or_default()
}

fn apply_marks(text: String, marks: &[Value]) -> String {
    marks.iter().fold(text, |acc, mark| {
        match mark["type"].as_str().unwrap_or("") {
            "bold"      => format!("**{acc}**"),
            "italic"    => format!("_{acc}_"),
            "underline" => acc,
            "strike"    => format!("~~{acc}~~"),
            "code"      => format!("`{acc}`"),
            "link" => {
                let href = mark["attrs"]["href"].as_str().unwrap_or("#");
                format!("[{acc}]({href})")
            }
            _ => acc,
        }
    })
}
