use super::{Inline, Mark};

#[derive(Debug, PartialEq)]
pub enum InlinePiece<'a> {
    Text(&'a str),
    Break,
    Name(&'a str),
    Marked(&'a Mark, Vec<InlinePiece<'a>>),
}

pub fn inline_tree(content: &[Inline]) -> Vec<InlinePiece<'_>> {
    let items: Vec<(Vec<&Mark>, &Inline)> = content
        .iter()
        .map(|inline| {
            let mut marks: Vec<&Mark> = match inline {
                Inline::Text { marks, .. } => marks.iter().collect(),
                Inline::Name { marks, .. } => marks.iter().collect(),
                Inline::HardBreak {} => Vec::new(),
            };
            marks.sort_by_key(|mark| mark.rank());
            (marks, inline)
        })
        .collect();
    group(&items, 0)
}

fn group<'a>(items: &[(Vec<&'a Mark>, &'a Inline)], depth: usize) -> Vec<InlinePiece<'a>> {
    let mut pieces = Vec::new();
    let mut index = 0;

    while index < items.len() {
        let (marks, inline) = &items[index];
        match marks.get(depth) {
            None => {
                pieces.push(match inline {
                    Inline::Text { text, .. } => InlinePiece::Text(text),
                    Inline::HardBreak {} => InlinePiece::Break,
                    Inline::Name { attrs, .. } => InlinePiece::Name(&attrs.fallback),
                });
                index += 1;
            }
            Some(mark) => {
                let end = items[index..]
                    .iter()
                    .position(|(other, _)| other.get(depth) != Some(mark))
                    .map_or(items.len(), |offset| index + offset);
                pieces.push(InlinePiece::Marked(mark, group(&items[index..end], depth + 1)));
                index = end;
            }
        }
    }

    pieces
}

pub fn words(content: &[Inline]) -> Vec<Inline> {
    let mut words = Vec::new();
    for inline in content {
        let Inline::Text { text, marks } = inline else {
            words.push(inline.clone());
            continue;
        };
        let mut word = String::new();
        let mut previous = None;
        for character in text.chars() {
            let class = if character.is_whitespace() { 0 } else if character.is_alphanumeric() { 1 } else { 2 };
            if previous.is_some_and(|previous| previous != class || class == 2) {
                words.push(Inline::Text { text: std::mem::take(&mut word), marks: marks.clone() });
            }
            previous = Some(class);
            word.push(character);
        }
        if !word.is_empty() {
            words.push(Inline::Text { text: word, marks: marks.clone() });
        }
    }
    words
}

pub fn join_words(words: Vec<Inline>) -> Vec<Inline> {
    let mut joined: Vec<Inline> = Vec::with_capacity(words.len());
    for word in words {
        if let (Some(Inline::Text { text, marks }), Inline::Text { text: next, marks: next_marks }) = (joined.last_mut(), &word) {
            if marks == next_marks {
                text.push_str(next);
                continue;
            }
        }
        joined.push(word);
    }
    joined
}
