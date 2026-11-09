use boutique::html;
use maud::Markup;

use crate::blocks::{music::MusicCard, youtube::YoutubeEmbed};
use crate::images::web_src;
use crate::renderer::Links;
use crate::document::{
    Block, Document, ImageAttrs, Inline, InlinePiece, ListItem, Mark, alt_text, code_text, figure_parts, inline_tree, quote_parts, requires_name,
};

fn rel(links: Links) -> &'static str {
    match links {
        Links::Trusted => "noopener noreferrer",
        Links::UserGenerated | Links::Inert => "nofollow ugc noopener noreferrer",
    }
}

pub fn render(document: &Document) -> Markup {
    render_with(document, Links::Trusted)
}

pub fn render_with(document: &Document, links: Links) -> Markup {
    html! {
        @for block in document.blocks() {
            @match block {
                Block::Music { attrs: music } => { MusicCard(music); }
                Block::Youtube { attrs: video } => { YoutubeEmbed(video); }
                _ => { (render_block(block, links)) }
            }
        }
    }
}

fn blocks(blocks: &[Block], links: Links) -> Markup {
    html! {
        @for block in blocks {
            (render_block(block, links))
        }
    }
}

fn render_block(block: &Block, links: Links) -> Markup {
    if block.requires_name() {
        return html! {};
    }
    match block {
        Block::Paragraph { content } | Block::Caption { content } | Block::Attribution { content } => html! { p { (inlines_with(content, links)) } },
        Block::Heading { attrs, content } => match attrs.level {
            3 => html! { h3 { (inlines_with(content, links)) } },
            4 => html! { h4 { (inlines_with(content, links)) } },
            _ => html! { h2 { (inlines_with(content, links)) } },
        },
        Block::Blockquote { content, .. } => html! { blockquote { (blocks(content, links)) } },
        Block::Pullquote { content } => {
            let (body, attribution) = quote_parts(content);
            html! {
                figure.pullquote {
                    blockquote { (blocks(body, links)) }
                    @if !attribution.is_empty() && !requires_name(attribution) {
                        figcaption { (inlines_with(attribution, links)) }
                    }
                }
            }
        }
        Block::BulletList { content } => html! { ul { (list_items(content, links)) } },
        Block::OrderedList { attrs, content } => {
            let start = (attrs.start != 1).then_some(attrs.start);
            html! { ol start=[start] { (list_items(content, links)) } }
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
                        figcaption { (inlines_with(caption, links)) }
                    }
                }
            },
            None => html! {},
        },
        Block::Boxout { attrs, content } => html! {
            aside.boxout data-style=(attrs.style.as_str()) data-width=(attrs.width.as_str()) { (blocks(content, links)) }
        },
        Block::Music { .. } | Block::Youtube { .. } => html! {},
    }
}

fn image(attrs: &ImageAttrs, alt: &str) -> Markup {
    html! { img src=(web_src(&attrs.src)) alt=(alt) title=[attrs.title.as_deref()]; }
}

fn list_items(items: &[ListItem], links: Links) -> Markup {
    html! {
        @for item in items {
            li { (blocks(&item.content, links)) }
        }
    }
}

pub(crate) fn inlines(content: &[Inline]) -> Markup {
    inlines_with(content, Links::Trusted)
}

fn inlines_with(content: &[Inline], links: Links) -> Markup {
    pieces(&inline_tree(content), links)
}

fn pieces(pieces: &[InlinePiece], links: Links) -> Markup {
    html! {
        @for piece in pieces {
            @match piece {
                InlinePiece::Text(text) => { (text) }
                InlinePiece::Break => { br; }
                InlinePiece::Name(fallback) => { (fallback) }
                InlinePiece::Marked(mark, children) => { (marked(mark, children, links)) }
            }
        }
    }
}

fn marked(mark: &Mark, children: &[InlinePiece], links: Links) -> Markup {
    let inner = pieces(children, links);
    match mark {
        Mark::Link { .. } if matches!(links, Links::Inert) => inner,
        Mark::Link { attrs } => html! { a href=(attrs.href) target="_blank" rel=(rel(links)) { (inner) } },
        Mark::Bold {} => html! { strong { (inner) } },
        Mark::Italic {} => html! { em { (inner) } },
        Mark::Underline {} => html! { u { (inner) } },
        Mark::Strike {} => html! { s { (inner) } },
        Mark::Code {} => html! { code { (inner) } },
    }
}
