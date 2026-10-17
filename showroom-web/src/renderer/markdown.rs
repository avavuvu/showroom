use crate::document::{
    Block, Document, ImageAttrs, Inline, InlinePiece, ListItem, Mark, MusicAttrs, MusicKind, code_text, figure_parts, inline_text, inline_tree, quote_parts,
};

pub fn render(document: &Document) -> String {
    blocks(document.blocks())
}

fn blocks(blocks: &[Block]) -> String {
    blocks.iter().map(block).filter(|text| !text.is_empty()).collect::<Vec<_>>().join("\n\n")
}

fn block(block: &Block) -> String {
    match block {
        Block::Paragraph { content } | Block::Caption { content } | Block::Attribution { content } => escape_line_starts(&inlines(content)),
        Block::Heading { attrs, content } => {
            let text = inlines(content);
            if text.trim().is_empty() {
                return String::new();
            }
            format!("{} {}", "#".repeat(attrs.level.into()), text.replace("\\\n", " "))
        }
        Block::Blockquote { content, .. } => quote(&blocks(content)),
        Block::Pullquote { content } => {
            let (body, attribution) = quote_parts(content);
            let mut text = blocks(body);
            if !attribution.is_empty() {
                text.push_str("\n\n\u{2014} ");
                text.push_str(&inlines(attribution));
            }
            quote(&text)
        }
        Block::BulletList { content } => content.iter().map(|item| list_item("-", item)).collect::<Vec<_>>().join("\n"),
        Block::OrderedList { attrs, content } => content
            .iter()
            .enumerate()
            .map(|(index, item)| list_item(&format!("{}.", attrs.start as usize + index), item))
            .collect::<Vec<_>>()
            .join("\n"),
        Block::CodeBlock { attrs, content } => {
            let text = code_text(content);
            let fence = if text.contains("```") { "~~~~" } else { "```" };
            format!("{fence}{}\n{text}\n{fence}", attrs.language.as_deref().unwrap_or(""))
        }
        Block::HorizontalRule {} => "---".to_string(),
        Block::Image { attrs } => image(attrs, &[]),
        Block::Figure { content, .. } => match figure_parts(content) {
            Some((attrs, caption)) => {
                let mut text = image(attrs, caption);
                if !caption.is_empty() {
                    text.push_str("\n\n");
                    text.push_str(&escape_line_starts(&inlines(caption)));
                }
                text
            }
            None => String::new(),
        },
        Block::Music { attrs } => music(attrs),
    }
}

fn music(attrs: &MusicAttrs) -> String {
    let mut heading = match (attrs.title.is_empty(), attrs.artist.is_empty()) {
        (false, false) => format!("**{}** \u{2014} {}", escape_text(&attrs.title), escape_text(&attrs.artist)),
        (false, true) => format!("**{}**", escape_text(&attrs.title)),
        (true, false) => escape_line_starts(&escape_text(&attrs.artist)),
        (true, true) => String::new(),
    };
    if attrs.kind == MusicKind::Album && !heading.is_empty() {
        heading.push_str(" (album)");
    }
    let links = attrs
        .links
        .iter()
        .map(|(service, link)| format!("[{}]({})", service.label(), destination(&link.url)))
        .collect::<Vec<_>>()
        .join(" \u{00b7} ");
    [heading, links].into_iter().filter(|text| !text.is_empty()).collect::<Vec<_>>().join("\n\n")
}

fn image(attrs: &ImageAttrs, caption: &[Inline]) -> String {
    format!("![{}]({})", escape_text(&inline_text(caption)), destination(&attrs.src))
}

fn quote(text: &str) -> String {
    text.lines()
        .map(|line| if line.is_empty() { ">".to_string() } else { format!("> {line}") })
        .collect::<Vec<_>>()
        .join("\n")
}

fn list_item(marker: &str, item: &ListItem) -> String {
    let body = blocks(&item.content);
    let indent = " ".repeat(marker.len() + 1);
    let mut lines = body.lines();
    let mut output = format!("{marker} {}", lines.next().unwrap_or(""));
    for line in lines {
        output.push('\n');
        if !line.is_empty() {
            output.push_str(&indent);
            output.push_str(line);
        }
    }
    output.trim_end().to_string()
}

fn inlines(content: &[Inline]) -> String {
    pieces(&inline_tree(content))
}

fn pieces(pieces: &[InlinePiece]) -> String {
    pieces
        .iter()
        .map(|piece| match piece {
            InlinePiece::Text(text) => escape_text(text),
            InlinePiece::Break => "\\\n".to_string(),
            InlinePiece::Marked(mark, children) => marked(mark, children),
        })
        .collect()
}

fn marked(mark: &Mark, children: &[InlinePiece]) -> String {
    match mark {
        Mark::Link { attrs } => format!("[{}]({})", self::pieces(children), destination(&attrs.href)),
        Mark::Bold {} => wrap(&self::pieces(children), "**"),
        Mark::Italic {} => wrap(&self::pieces(children), "*"),
        Mark::Strike {} => wrap(&self::pieces(children), "~~"),
        Mark::Underline {} => self::pieces(children),
        Mark::Code {} => code_span(&raw_text(children)),
    }
}

fn wrap(text: &str, delimiter: &str) -> String {
    let core = text.trim();
    if core.is_empty() {
        return text.to_string();
    }
    let start = text.len() - text.trim_start().len();
    let end = start + core.len();
    format!("{}{delimiter}{core}{delimiter}{}", &text[..start], &text[end..])
}

fn raw_text(pieces: &[InlinePiece]) -> String {
    pieces
        .iter()
        .map(|piece| match piece {
            InlinePiece::Text(text) => text.to_string(),
            InlinePiece::Break => " ".to_string(),
            InlinePiece::Marked(_, children) => raw_text(children),
        })
        .collect()
}

fn code_span(text: &str) -> String {
    let mut longest = 0;
    let mut current = 0;
    for character in text.chars() {
        if character == '`' {
            current += 1;
            longest = longest.max(current);
        } else {
            current = 0;
        }
    }
    let fence = "`".repeat(longest + 1);
    let padding = if text.starts_with('`') || text.ends_with('`') { " " } else { "" };
    format!("{fence}{padding}{text}{padding}{fence}")
}

fn destination(url: &str) -> String {
    if url.contains([' ', '(', ')', '<', '>']) {
        format!("<{}>", url.replace('<', "%3C").replace('>', "%3E"))
    } else {
        url.to_string()
    }
}

fn escape_text(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    for character in text.chars() {
        if "\\`*_[]<>#~|".contains(character) {
            output.push('\\');
        }
        output.push(character);
    }
    output
}

fn escape_line_starts(text: &str) -> String {
    text.split('\n').map(escape_line_start).collect::<Vec<_>>().join("\n")
}

fn escape_line_start(line: &str) -> String {
    if line.starts_with(['-', '+', '=']) {
        return format!("\\{line}");
    }
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    if digits > 0 && line[digits..].starts_with(['.', ')']) {
        return format!("{}\\{}", &line[..digits], &line[digits..]);
    }
    line.to_string()
}
