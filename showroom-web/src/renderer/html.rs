use boutique::html;
use maud::Markup;

use crate::blocks::music::{MusicCard, Tone};
use crate::document::{
    Block, Document, ImageAttrs, Inline, InlinePiece, ListItem, Mark, alt_text, code_text, figure_parts, inline_tree, quote_parts, requires_name,
};

pub fn render(document: &Document, tone: Tone) -> Markup {
    html! {
        @for block in document.blocks() {
            @match block {
                Block::Music { attrs: music } => { MusicCard(music, tone = tone); }
                _ => { (render_block(block)) }
            }
        }
    }
}

fn blocks(blocks: &[Block]) -> Markup {
    html! {
        @for block in blocks {
            (render_block(block))
        }
    }
}

fn render_block(block: &Block) -> Markup {
    if block.requires_name() {
        return html! {};
    }
    match block {
        Block::Paragraph { content } | Block::Caption { content } | Block::Attribution { content } => html! { p { (inlines(content)) } },
        Block::Heading { attrs, content } => match attrs.level {
            3 => html! { h3 { (inlines(content)) } },
            4 => html! { h4 { (inlines(content)) } },
            _ => html! { h2 { (inlines(content)) } },
        },
        Block::Blockquote { content, .. } => html! { blockquote { (blocks(content)) } },
        Block::Pullquote { content } => {
            let (body, attribution) = quote_parts(content);
            html! {
                figure.pullquote {
                    blockquote { (blocks(body)) }
                    @if !attribution.is_empty() && !requires_name(attribution) {
                        figcaption { (inlines(attribution)) }
                    }
                }
            }
        }
        Block::BulletList { content } => html! { ul { (list_items(content)) } },
        Block::OrderedList { attrs, content } => {
            let start = (attrs.start != 1).then_some(attrs.start);
            html! { ol start=[start] { (list_items(content)) } }
        }
        Block::CodeBlock { attrs, content } => {
            let class = attrs.language.as_ref().map(|language| format!("language-{language}"));
            html! { pre { code class=[class] { (code_text(content)) } } }
        }
        Block::HorizontalRule {} => html! { hr; },
        Block::Image { attrs } => image(attrs, ""),
        Block::Figure { attrs, content } => match figure_parts(content) {
            Some((image_attrs, caption)) => html! {
                figure data-width=(attrs.width.as_str()) {
                    (image(image_attrs, &alt_text(caption)))
                    @if !caption.is_empty() && !requires_name(caption) {
                        figcaption { (inlines(caption)) }
                    }
                }
            },
            None => html! {},
        },
        Block::Music { .. } => html! {},
    }
}

fn image(attrs: &ImageAttrs, alt: &str) -> Markup {
    html! { img src=(attrs.src) alt=(alt) title=[attrs.title.as_deref()]; }
}

fn list_items(items: &[ListItem]) -> Markup {
    html! {
        @for item in items {
            li { (blocks(&item.content)) }
        }
    }
}

fn inlines(content: &[Inline]) -> Markup {
    pieces(&inline_tree(content))
}

fn pieces(pieces: &[InlinePiece]) -> Markup {
    html! {
        @for piece in pieces {
            @match piece {
                InlinePiece::Text(text) => { (text) }
                InlinePiece::Break => { br; }
                InlinePiece::Name(fallback) => { (fallback) }
                InlinePiece::Marked(mark, children) => { (marked(mark, children)) }
            }
        }
    }
}

fn marked(mark: &Mark, children: &[InlinePiece]) -> Markup {
    let inner = pieces(children);
    match mark {
        Mark::Link { attrs } => html! { a href=(attrs.href) target="_blank" rel="noopener noreferrer" { (inner) } },
        Mark::Bold {} => html! { strong { (inner) } },
        Mark::Italic {} => html! { em { (inner) } },
        Mark::Underline {} => html! { u { (inner) } },
        Mark::Strike {} => html! { s { (inner) } },
        Mark::Code {} => html! { code { (inner) } },
    }
}
