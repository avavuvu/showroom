use super::{Inline, Mark};

#[derive(Debug, PartialEq)]
pub enum InlinePiece<'a> {
    Text(&'a str),
    Break,
    Marked(&'a Mark, Vec<InlinePiece<'a>>),
}

pub fn inline_tree(content: &[Inline]) -> Vec<InlinePiece<'_>> {
    let items: Vec<(Vec<&Mark>, &Inline)> = content
        .iter()
        .map(|inline| {
            let mut marks: Vec<&Mark> = match inline {
                Inline::Text { marks, .. } => marks.iter().collect(),
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
