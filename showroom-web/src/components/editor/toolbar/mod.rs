use boutique::html;
use bq_components::component;
use maud::Markup;

use crate::components::{
    Menu,
    icon::{Glyph, Icon},
};


struct Tool {
    command: &'static str,
    label: &'static str,
    shortcut: &'static str,
    glyph: Glyph,
    toggle: bool,
    secondary: bool,
}

const fn tool(command: &'static str, label: &'static str, shortcut: &'static str, glyph: Glyph, toggle: bool, secondary: bool) -> Tool {
    Tool { command, label, shortcut, glyph, toggle, secondary }
}

const HISTORY: &[Tool] = &[
    tool("undo", "Undo", "Mod-z", Glyph::Undo, false, false),
    tool("redo", "Redo", "Shift-Mod-z", Glyph::Redo, false, true),
];

const GROUPS: &[&[Tool]] = &[
    &[
        tool("bold", "Bold", "Mod-b", Glyph::Bold, true, false),
        tool("italic", "Italic", "Mod-i", Glyph::Italic, true, false),
        tool("underline", "Underline", "Mod-u", Glyph::Underline, true, true),
        tool("strike", "Strikethrough", "Shift-Mod-x", Glyph::Strikethrough, true, true),
        tool("code", "Inline code", "Mod-e", Glyph::Code, true, true),
        tool("link", "Link", "Mod-k", Glyph::Link, true, false),
    ],
    &[
        tool("bullet-list", "Bulleted list", "Shift-Mod-8", Glyph::List, true, false),
        tool("ordered-list", "Numbered list", "Shift-Mod-7", Glyph::ListOrdered, true, true),
        tool("blockquote", "Quote", "Shift-Mod-9", Glyph::Quote, true, false),
        tool("pullquote", "Pull quote", "", Glyph::TextQuote, true, true),
    ],
    &[
        tool("horizontal-rule", "Divider", "", Glyph::Minus, false, true),
        tool("image", "Image", "", Glyph::Image, false, false),
    ],
];

const BLOCKS: &[(&str, &str, &str, Glyph)] = &[
    ("paragraph", "Paragraph", "Mod-Alt-0", Glyph::Pilcrow),
    ("heading-2", "Heading", "Mod-Alt-2", Glyph::Heading2),
    ("heading-3", "Subheading", "Mod-Alt-3", Glyph::Heading3),
    ("heading-4", "Minor heading", "Mod-Alt-4", Glyph::Heading4),
    ("code-block", "Code block", "Mod-Alt-c", Glyph::CodeXml),
];

const OTHER_BLOCKS: &[(&str, &str)] = &[("caption", "Caption"), ("attribution", "Source"), ("mixed", "Mixed")];

#[component(setup = "editor-toolbar")]
pub fn toolbar() -> Markup {
    let secondary = || std::iter::once(HISTORY).chain(GROUPS.iter().copied()).flatten().filter(|tool| tool.secondary);
    html! {
        div.toolbar
            role="toolbar"
            aria-label="Formatting"
            bq-setup
            bq-on:mousedown|prevent
            bq-on:keydown="navigate"
            bq-on:focusin="track"
        {
            div.toolbar-group {
                @for tool in HISTORY { (toolbar_button(tool)) }
            }
            div.toolbar-group {
                (block_menu())
            }
            @for group in GROUPS {
                div.toolbar-group {
                    @for tool in *group { (toolbar_button(tool)) }
                }
            }
            div.toolbar-group.more-group {
                Menu("More formatting", id = "editor-more", button = html! { Icon(Glyph::Ellipsis); })
                    .toolbar-button.more-button
                    tabindex="-1"
                {
                    @for tool in secondary() { (more_option(tool)) }
                }
            }
        }
    }
}

fn shortcut_attr(shortcut: &str) -> Option<&str> {
    (!shortcut.is_empty()).then_some(shortcut)
}

fn toolbar_button(tool: &Tool) -> Markup {
    html! {
        button.toolbar-button.secondary-tool[tool.secondary]
            type="button"
            tabindex="-1"
            bq-ref="command"
            bq-on:click="run-command"
            data-command=(tool.command)
            data-shortcut=[shortcut_attr(tool.shortcut)]
            aria-label=(tool.label)
            aria-pressed=[tool.toggle.then_some("false")]
            title=(tool.label)
        {
            Icon(tool.glyph);
        }
    }
}

fn more_option(tool: &Tool) -> Markup {
    html! {
        button.menu-option
            type="button"
            role=(if tool.toggle { "menuitemcheckbox" } else { "menuitem" })
            aria-checked=[tool.toggle.then_some("false")]
            tabindex="-1"
            bq-ref="editor-toolbar.command"
            bq-on:click="editor-toolbar.run-command"
            data-command=(tool.command)
            data-shortcut=[shortcut_attr(tool.shortcut)]
        {
            Icon(tool.glyph);
            span.option-label { (tool.label) }
            span.shortcut aria-hidden="true" {}
        }
    }
}

fn block_menu() -> Markup {
    let current = html! {
        span.block-current {
            @for (index, (value, label, _, glyph)) in BLOCKS.iter().enumerate() {
                span.block-label data-block=(value) hidden[index > 0] bq-ref="editor-toolbar.block-label" {
                    Icon(*glyph);
                    span.option-label { (label) }
                }
            }
            @for (value, label) in OTHER_BLOCKS {
                span.block-label data-block=(value) hidden bq-ref="editor-toolbar.block-label" {
                    span.option-label { (label) }
                }
            }
        }
        Icon(Glyph::ChevronDown);
    };
    html! {
        Menu("Text style", id = "editor-block", button = current) .block-button tabindex="-1" bq-ref="editor-toolbar.block-button" {
            @for (value, label, shortcut, glyph) in BLOCKS {
                button.menu-option
                    type="button"
                    role="menuitemradio"
                    aria-checked="false"
                    tabindex="-1"
                    data-block=(value)
                    data-shortcut=(shortcut)
                    bq-ref="editor-toolbar.block-option"
                    bq-on:click="editor-toolbar.set-block"
                {
                    Icon(*glyph);
                    span.option-label { (label) }
                    span.shortcut aria-hidden="true" {}
                }
            }
        }
    }
}
