use boutique::html;
use maud::Markup;

use crate::document::{Block, Document, ImageAttrs, Inline, InlinePiece, ListItem, Mark, code_text, figure_parts, inline_tree, quote_parts};

pub fn render(document: &Document) -> Markup {
    blocks(document.blocks())
}

fn blocks(blocks: &[Block]) -> Markup {
    html! {
        @for block in blocks {
            (render_block(block))
        }
    }
}

fn render_block(block: &Block) -> Markup {
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
                    @if !attribution.is_empty() {
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
        Block::Image { attrs } => image(attrs),
        Block::Figure { attrs, content } => match figure_parts(content) {
            Some((image_attrs, caption)) => html! {
                figure data-width=(attrs.width.as_str()) {
                    (image(image_attrs))
                    @if !caption.is_empty() {
                        figcaption { (inlines(caption)) }
                    }
                }
            },
            None => html! {},
        },
    }
}

fn image(attrs: &ImageAttrs) -> Markup {
    html! { img src=(attrs.src) alt="" title=[attrs.title.as_deref()]; }
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::render;
    use crate::document::Document;

    fn html(value: serde_json::Value) -> String {
        render(&Document::from_value(&value).expect("valid document")).into_string()
    }

    fn doc(content: serde_json::Value) -> String {
        html(json!({ "type": "doc", "content": content }))
    }

    fn text(text: &str, marks: serde_json::Value) -> serde_json::Value {
        json!({ "type": "text", "text": text, "marks": marks })
    }

    #[test]
    fn paragraphs_and_escaping() {
        let output = doc(json!([
            { "type": "paragraph", "content": [{ "type": "text", "text": "<script>alert(\"x\")</script> & co" }] }
        ]));
        assert_eq!(output, "<p>&lt;script&gt;alert(&quot;x&quot;)&lt;/script&gt; &amp; co</p>");
    }

    #[test]
    fn empty_document_renders_nothing() {
        assert_eq!(html(json!({ "type": "doc", "content": [] })), "");
        assert_eq!(doc(json!([{ "type": "paragraph" }, { "type": "paragraph" }])), "");
    }

    #[test]
    fn trailing_empty_paragraphs_are_dropped_but_inner_ones_stay() {
        let output = doc(json!([
            { "type": "paragraph", "content": [{ "type": "text", "text": "a" }] },
            { "type": "paragraph" },
            { "type": "paragraph", "content": [{ "type": "text", "text": "b" }] },
            { "type": "paragraph" }
        ]));
        assert_eq!(output, "<p>a</p><p></p><p>b</p>");
    }

    #[test]
    fn headings_are_level_two_to_four() {
        let output = doc(json!([
            { "type": "heading", "attrs": { "level": 1 }, "content": [{ "type": "text", "text": "One" }] },
            { "type": "heading", "attrs": { "level": 2 }, "content": [{ "type": "text", "text": "Two" }] },
            { "type": "heading", "attrs": { "level": 3 }, "content": [{ "type": "text", "text": "Three" }] },
            { "type": "heading", "attrs": { "level": 4 }, "content": [{ "type": "text", "text": "Four" }] },
            { "type": "heading", "attrs": { "level": 6 }, "content": [{ "type": "text", "text": "Six" }] }
        ]));
        assert_eq!(output, "<h2>One</h2><h2>Two</h2><h3>Three</h3><h4>Four</h4><h4>Six</h4>");
    }

    #[test]
    fn marks_nest_in_a_fixed_order() {
        let output = doc(json!([
            { "type": "paragraph", "content": [text("x", json!([{ "type": "code" }, { "type": "bold" }, { "type": "italic" }]))] }
        ]));
        assert_eq!(output, "<p><strong><em><code>x</code></em></strong></p>");
    }

    #[test]
    fn a_link_over_mixed_marks_is_one_anchor() {
        let link = json!({ "type": "link", "attrs": { "href": "https://example.com/?a=1&b=2", "target": "_self\" onclick=\"x", "rel": null, "class": null } });
        let output = doc(json!([
            { "type": "paragraph", "content": [
                text("read ", json!([link.clone()])),
                text("this", json!([link.clone(), { "type": "bold" }])),
                { "type": "text", "text": " now" }
            ] }
        ]));
        assert_eq!(
            output,
            "<p><a href=\"https://example.com/?a=1&amp;b=2\" target=\"_blank\" rel=\"noopener noreferrer\">read <strong>this</strong></a> now</p>"
        );
    }

    #[test]
    fn unsafe_and_empty_links_lose_the_link_but_keep_the_text() {
        let output = doc(json!([
            { "type": "paragraph", "content": [
                text("a", json!([{ "type": "link", "attrs": { "href": "javascript:alert(1)" } }])),
                text("b", json!([{ "type": "link", "attrs": { "href": "" } }])),
                text("c", json!([{ "type": "link", "attrs": { "href": null } }])),
                text("d", json!([{ "type": "link", "attrs": { "href": "mailto:ava@example.com" } }]))
            ] }
        ]));
        assert_eq!(output, "<p>abc<a href=\"mailto:ava@example.com\" target=\"_blank\" rel=\"noopener noreferrer\">d</a></p>");
    }

    #[test]
    fn hard_breaks() {
        let output = doc(json!([
            { "type": "paragraph", "content": [{ "type": "text", "text": "a" }, { "type": "hardBreak" }, { "type": "text", "text": "b" }] }
        ]));
        assert_eq!(output, "<p>a<br>b</p>");
    }

    #[test]
    fn quotes_and_attributed_quotes() {
        let paragraph = json!({ "type": "paragraph", "content": [{ "type": "text", "text": "q" }] });
        let output = doc(json!([
            { "type": "blockquote", "content": [paragraph.clone()] },
            { "type": "blockquote", "attrs": { "variant": "other" }, "content": [paragraph.clone()] },
            { "type": "quote", "content": [paragraph.clone(), { "type": "attribution", "content": [
                { "type": "text", "text": "Ava, " },
                { "type": "text", "text": "Notes", "marks": [{ "type": "italic" }, { "type": "link", "attrs": { "href": "https://example.com" } }] }
            ] }] },
            { "type": "quote", "content": [paragraph.clone()] }
        ]));
        assert_eq!(
            output,
            "<blockquote><p>q</p></blockquote><blockquote><p>q</p></blockquote>\
             <figure class=\"pullquote\"><blockquote><p>q</p></blockquote><figcaption>Ava, <a href=\"https://example.com\" target=\"_blank\" rel=\"noopener noreferrer\"><em>Notes</em></a></figcaption></figure>\
             <figure class=\"pullquote\"><blockquote><p>q</p></blockquote></figure>"
        );
    }

    #[test]
    fn old_pull_quotes_become_quotes_without_a_source() {
        let output = doc(json!([
            { "type": "blockquote", "attrs": { "variant": "pull" }, "content": [{ "type": "paragraph", "content": [{ "type": "text", "text": "q" }] }] }
        ]));
        assert_eq!(output, "<figure class=\"pullquote\"><blockquote><p>q</p></blockquote></figure>");
    }

    #[test]
    fn lists() {
        let item = |value: &str| json!({ "type": "listItem", "content": [{ "type": "paragraph", "content": [{ "type": "text", "text": value }] }] });
        let output = doc(json!([
            { "type": "bulletList", "content": [item("a"), {
                "type": "listItem",
                "content": [
                    { "type": "paragraph", "content": [{ "type": "text", "text": "b" }] },
                    { "type": "orderedList", "attrs": { "start": 1, "type": null }, "content": [item("c")] }
                ]
            }] },
            { "type": "orderedList", "attrs": { "start": 3 }, "content": [item("d")] },
            { "type": "bulletList", "content": [] }
        ]));
        assert_eq!(
            output,
            "<ul><li><p>a</p></li><li><p>b</p><ol><li><p>c</p></li></ol></li></ul><ol start=\"3\"><li><p>d</p></li></ol>"
        );
    }

    #[test]
    fn code_blocks() {
        let output = doc(json!([
            { "type": "codeBlock", "attrs": { "language": "rust" }, "content": [{ "type": "text", "text": "let a = 1 < 2;" }] },
            { "type": "codeBlock", "attrs": { "language": null }, "content": [{ "type": "text", "text": "plain" }] },
            { "type": "codeBlock", "attrs": { "language": "x\" onload=\"y" }, "content": [{ "type": "text", "text": "z" }] }
        ]));
        assert_eq!(
            output,
            "<pre><code class=\"language-rust\">let a = 1 &lt; 2;</code></pre><pre><code>plain</code></pre><pre><code>z</code></pre>"
        );
    }

    #[test]
    fn old_images_become_figures() {
        let output = doc(json!([
            { "type": "image", "attrs": { "src": "https://res.cloudinary.com/demo/a.jpg", "caption": " A <b>\"cat\"</b> ", "title": null, "width": "full", "publicId": "a" } },
            { "type": "horizontalRule" },
            { "type": "image", "attrs": { "src": "https://res.cloudinary.com/demo/b.jpg", "alt": "IMG_1234.jpg", "caption": "  ", "title": "B", "width": null } },
            { "type": "image", "attrs": { "src": "", "caption": "empty" } },
            { "type": "image", "attrs": { "src": "javascript:alert(1)" } }
        ]));
        assert_eq!(
            output,
            "<figure data-width=\"full\"><img src=\"https://res.cloudinary.com/demo/a.jpg\" alt=\"\"><figcaption>A &lt;b&gt;&quot;cat&quot;&lt;/b&gt;</figcaption></figure>\
             <hr>\
             <figure data-width=\"normal\"><img src=\"https://res.cloudinary.com/demo/b.jpg\" alt=\"\" title=\"B\"></figure>"
        );
    }

    #[test]
    fn figure_captions_keep_their_styles() {
        let output = doc(json!([
            { "type": "figure", "attrs": { "width": "normal" }, "content": [
                { "type": "image", "attrs": { "src": "https://res.cloudinary.com/demo/a.jpg", "publicId": "a" } },
                { "type": "caption", "content": [
                    { "type": "text", "text": " Photo by " },
                    { "type": "text", "text": "Ava", "marks": [{ "type": "bold" }] },
                    { "type": "text", "text": " " }
                ] }
            ] },
            { "type": "figure", "content": [{ "type": "caption", "content": [{ "type": "text", "text": "no image" }] }] }
        ]));
        assert_eq!(
            output,
            "<figure data-width=\"normal\"><img src=\"https://res.cloudinary.com/demo/a.jpg\" alt=\"\"><figcaption>Photo by <strong>Ava</strong></figcaption></figure>"
        );
    }

    #[test]
    fn normalized_figures_have_an_image_and_a_caption() {
        let document = Document::from_value(&json!({ "type": "doc", "content": [
            { "type": "image", "attrs": { "src": "https://res.cloudinary.com/demo/a.jpg" } }
        ] }))
        .expect("valid document");
        assert_eq!(
            document.to_value(),
            json!({ "type": "doc", "content": [{ "type": "figure", "attrs": { "width": "normal" }, "content": [
                { "type": "image", "attrs": { "src": "https://res.cloudinary.com/demo/a.jpg" } },
                { "type": "caption" }
            ] }] })
        );
    }

    #[test]
    fn unknown_node_types_are_an_error() {
        let value = json!({ "type": "doc", "content": [{ "type": "table", "content": [] }] });
        assert!(Document::from_value(&value).is_err());
    }
}
