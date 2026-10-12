use boutique::html;
use bq_components::{component, setup};
use maud::Markup;

use crate::components::{
    Menu,
    icon::{Glyph, Icon},
};

setup!(EditorToolbar);

#[component]
pub fn toolbar() -> Markup {
    html! {
        div.toolbar
            role="toolbar"
            aria-label="Formatting"
            bq-setup=(EditorToolbar)
            bq-on:mousedown|prevent
            bq-on:keydown="navigate"
            bq-on:focusin="track"
        {
            div.toolbar-group {
                (toolbar_button("undo", "Undo", "Mod-z", Glyph::Undo, false))
                (toolbar_button("redo", "Redo", "Shift-Mod-z", Glyph::Redo, true))
            }

            div.toolbar-group {
                (block_menu())
            }

            div.toolbar-group {
                (toolbar_button("bold", "Bold", "Mod-b", Glyph::Bold, false))
                (toolbar_button("italic", "Italic", "Mod-i", Glyph::Italic, false))
                (toolbar_button("underline", "Underline", "Mod-u", Glyph::Underline, true))
                (toolbar_button("strike", "Strikethrough", "Shift-Mod-x", Glyph::Strikethrough, true))
                (toolbar_button("code", "Inline code", "Mod-e", Glyph::Code, true))
                (toolbar_button("link", "Link", "Mod-k", Glyph::Link, false))
            }

            div.toolbar-group {
                (toolbar_button("bullet-list", "Bulleted list", "Shift-Mod-8", Glyph::List, false))
                (toolbar_button("ordered-list", "Numbered list", "Shift-Mod-7", Glyph::ListOrdered, true))
                (toolbar_button("blockquote", "Quote", "Shift-Mod-9", Glyph::Quote, false))
                (toolbar_button("pullquote", "Pull quote", "", Glyph::TextQuote, true))
            }

            div.toolbar-group {
                (toolbar_button("horizontal-rule", "Divider", "", Glyph::Minus, true))
                (toolbar_button("image", "Image", "", Glyph::Image, false))
            }

            div.toolbar-group.more-group {
                Menu("More formatting", anchor = ".editor-toolbar", button = html! { Icon(Glyph::Ellipsis); })
                    .toolbar-button.more-button
                    tabindex="-1"
                {
                    (more_option("redo", "Redo", "Shift-Mod-z", Glyph::Redo))
                    (more_option("underline", "Underline", "Mod-u", Glyph::Underline))
                    (more_option("strike", "Strikethrough", "Shift-Mod-x", Glyph::Strikethrough))
                    (more_option("code", "Inline code", "Mod-e", Glyph::Code))
                    (more_option("ordered-list", "Numbered list", "Shift-Mod-7", Glyph::ListOrdered))
                    (more_option("pullquote", "Pull quote", "", Glyph::TextQuote))
                    (more_option("horizontal-rule", "Divider", "", Glyph::Minus))
                }
            }
        }
    }
}

fn more_option(command: &str, label: &str, shortcut: &str, glyph: Glyph) -> Markup {
    html! {
        button.menu-option
            type="button"
            role="menuitem"
            tabindex="-1"
            bq-ref="command"
            bq-on:click="run-command"
            data-command=(command)
            data-shortcut=[(!shortcut.is_empty()).then_some(shortcut)]
        {
            Icon(glyph);
            span.option-label { (label) }
            span.shortcut aria-hidden="true" {}
        }
    }
}

fn block_menu() -> Markup {
    let current = html! {
        span.block-current bq-ref="block-current" {
            Icon(Glyph::Pilcrow);
            span.option-label { "Paragraph" }
        }
        Icon(Glyph::ChevronDown);
    };
    html! {
        Menu("Text style", anchor = ".editor-toolbar", button = current) .block-button tabindex="-1" bq-ref="block-button" {
            (block_option("paragraph", "Paragraph", "Mod-Alt-0", Glyph::Pilcrow))
            (block_option("heading-2", "Heading", "Mod-Alt-2", Glyph::Heading2))
            (block_option("heading-3", "Subheading", "Mod-Alt-3", Glyph::Heading3))
            (block_option("heading-4", "Minor heading", "Mod-Alt-4", Glyph::Heading4))
            (block_option("code-block", "Code block", "Mod-Alt-c", Glyph::CodeXml))
        }
    }
}

fn block_option(value: &str, label: &str, shortcut: &str, glyph: Glyph) -> Markup {
    html! {
        button.menu-option
            type="button"
            role="menuitemradio"
            aria-checked="false"
            tabindex="-1"
            data-block=(value)
            data-shortcut=(shortcut)
            bq-ref="block-option"
            bq-on:click="set-block"
        {
            Icon(glyph);
            span.option-label { (label) }
            span.shortcut aria-hidden="true" {}
        }
    }
}

fn toolbar_button(command: &str, label: &str, shortcut: &str, glyph: Glyph, secondary: bool) -> Markup {
    html! {
        button.toolbar-button.secondary-tool[secondary]
            type="button"
            tabindex="-1"
            bq-ref="command"
            bq-on:click="run-command"
            data-command=(command)
            data-shortcut=[(!shortcut.is_empty()).then_some(shortcut)]
            aria-label=(label)
            title=(label)
        {
            Icon(glyph);
        }
    }
}
