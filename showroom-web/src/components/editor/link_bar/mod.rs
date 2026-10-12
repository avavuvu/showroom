use boutique::html;
use bq_components::component;
use maud::Markup;

use crate::components::icon::{Glyph, Icon};

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
