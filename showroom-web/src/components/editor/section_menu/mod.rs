use boutique::html;
use bq_components::component;
use maud::Markup;

use crate::components::{
    Menu,
    icon::{Glyph, Icon},
};

fn option(action: &str, glyph: Glyph, label: &str) -> Markup {
    html! {
        button.menu-option
            type="button"
            role="menuitem"
            tabindex="-1"
            data-action=(action)
            bq-ref="editor-section-menu.option"
            bq-on:click="editor-section-menu.choose"
        {
            Icon(glyph);
            span.option-label { (label) }
            span.shortcut aria-hidden="true" {}
        }
    }
}

#[component(setup = "editor-section-menu")]
pub fn section_menu() -> Markup {
    html! {
        div.section-menu.prose-overlay bq-setup {
            div.section-outline aria-hidden="true" bq-ref="outline" {}
            div.section-drop aria-hidden="true" bq-ref="drop" {}
            div.section-handle bq-ref="handle" {
                Menu("Section", id = "editor-section", button = html! {
                    span.section-icon data-level="2" { Icon(Glyph::Heading2); }
                    span.section-icon data-level="3" { Icon(Glyph::Heading3); }
                    span.section-icon data-level="4" { Icon(Glyph::Heading4); }
                })
                    .section-button
                    tabindex="-1"
                    bq-ref="editor-section-menu.button"
                {
                    (option("select", Glyph::TextBox, "Select section"))
                    (option("send", Glyph::Users, "Send for suggestions\u{2026}"))
                    (option("reviews", Glyph::Layers, "See suggestions"))
                }
            }
        }
    }
}
