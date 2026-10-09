use boutique::html;
use bq_components::component;
use maud::Markup;

use crate::components::icon::{Glyph, Icon};

#[component]
pub fn toolbar() -> Markup {
    html! {
        div.toolbar role="toolbar" aria-label="Formatting" bq-ref="toolbar" {
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
                button.toolbar-button.more-button
                    type="button"
                    tabindex="-1"
                    aria-haspopup="menu"
                    aria-expanded="false"
                    aria-label="More formatting"
                    title="More formatting"
                    bq-ref="more-button"
                {
                    Icon(Glyph::Ellipsis);
                }
                div.menu-panel role="menu" aria-label="More formatting" hidden bq-ref="more-options" {
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
            data-command=(command)
            data-shortcut=[(!shortcut.is_empty()).then_some(shortcut)]
        {
            Icon(glyph);
            span.option-label { (label) }
            span.shortcut aria-hidden="true" {}
        }
    }
}

#[component]
pub fn link_bar() -> Markup {
    html! {
        div.link-bar.is-idle bq-ref="link-bar" {
            input.link-input.link-text
                type="text"
                name="text"
                placeholder="Display text"
                aria-label="Display text"
                autocomplete="off"
                bq-ref="link-text";
            input.link-input.link-url
                type="text"
                inputmode="url"
                name="href"
                placeholder="https://example.com"
                aria-label="Link address"
                autocomplete="off"
                spellcheck="false"
                bq-ref="link-input";
            a.icon-button
                href="#"
                target="_blank"
                rel="noopener noreferrer"
                aria-label="Open link in a new tab"
                title="Open link in a new tab"
                aria-disabled="true"
                bq-ref="link-open"
            {
                Icon(Glyph::ExternalLink);
            }
            button.icon-button type="button" aria-label="Remove link" title="Remove link" disabled bq-ref="link-remove" {
                Icon(Glyph::Unlink);
            }
        }
    }
}

fn block_menu() -> Markup {
    html! {
        div.block-menu {
            button.block-button
                type="button"
                tabindex="-1"
                aria-haspopup="menu"
                aria-expanded="false"
                aria-label="Text style"
                title="Text style"
                bq-ref="block-button"
            {
                span.block-current bq-ref="block-current" {
                    Icon(Glyph::Pilcrow);
                    span.option-label { "Paragraph" }
                }
                Icon(Glyph::ChevronDown);
            }
            div.menu-panel role="menu" aria-label="Text style" hidden bq-ref="block-options" {
                (block_option("paragraph", "Paragraph", "Mod-Alt-0", Glyph::Pilcrow))
                (block_option("heading-2", "Heading", "Mod-Alt-2", Glyph::Heading2))
                (block_option("heading-3", "Subheading", "Mod-Alt-3", Glyph::Heading3))
                (block_option("heading-4", "Minor heading", "Mod-Alt-4", Glyph::Heading4))
                (block_option("code-block", "Code block", "Mod-Alt-c", Glyph::CodeXml))
            }
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
            data-command=(command)
            data-shortcut=[(!shortcut.is_empty()).then_some(shortcut)]
            aria-label=(label)
            title=(label)
        {
            Icon(glyph);
        }
    }
}
