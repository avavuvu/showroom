use boutique::html;
use maud::Markup;
use similar::DiffTag;

use super::html::{inlines, render};
use crate::document::{Block, Document, Inline, merge::ops, words};

fn wrap(block: &Block, content: Markup) -> Markup {
    match block {
        Block::Heading { attrs, .. } => match attrs.level {
            3 => html! { h3 { (content) } },
            4 => html! { h4 { (content) } },
            _ => html! { h2 { (content) } },
        },
        _ => html! { p { (content) } },
    }
}

fn text_diff(old: &[Inline], new: &[Inline]) -> Markup {
    let (old, new) = (words(old), words(new));
    html! {
        @for op in ops(&old, &new) {
            @match op.tag() {
                DiffTag::Equal => { (inlines(&new[op.new_range()])) }
                DiffTag::Delete => { del { (inlines(&old[op.old_range()])) } }
                DiffTag::Insert => { ins { (inlines(&new[op.new_range()])) } }
                DiffTag::Replace => {
                    del { (inlines(&old[op.old_range()])) }
                    ins { (inlines(&new[op.new_range()])) }
                }
            }
        }
    }
}

fn one(block: &Block) -> Markup {
    render(&Document { content: vec![block.clone()] })
}

pub fn blocks(blocks: &[Block]) -> Markup {
    render(&Document { content: blocks.to_vec() })
}

pub fn change(from: &[Block], to: &[Block]) -> Markup {
    html! {
        @for index in 0..from.len().max(to.len()) {
            @match (from.get(index), to.get(index)) {
                (Some(old), Some(new)) if old.same_text_kind(new) => {
                    (wrap(new, text_diff(old.inline_content().unwrap_or_default(), new.inline_content().unwrap_or_default())))
                }
                (old, new) => {
                    @if let Some(old) = old { del.diff-block.is-media[old.is_card()] { (one(old)) } }
                    @if let Some(new) = new { ins.diff-block.is-media[new.is_card()] { (one(new)) } }
                }
            }
        }
    }
}
