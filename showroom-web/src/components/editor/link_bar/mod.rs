use boutique::html;
use bq_components::{component, setup};
use maud::Markup;

use crate::components::icon::{Glyph, Icon};

setup!(EditorLinkBar);

#[component]
pub fn link_bar() -> Markup {
    html! {
        div.link-bar.is-idle bq-setup=(EditorLinkBar) {
            input.link-input.link-text
                type="text"
                name="text"
                placeholder="Display text"
                aria-label="Display text"
                autocomplete="off"
                enterkeyhint="done"
                bq-ref="text"
                bq-on:focus="start"
                bq-on:blur="leave"
                bq-on:keydown|escape|prevent="cancel"
                bq-on:keydown|enter|prevent="apply";
            input.link-input.link-url
                type="text"
                inputmode="url"
                name="href"
                placeholder="https://example.com"
                aria-label="Link address"
                autocomplete="off"
                autocapitalize="off"
                autocorrect="off"
                spellcheck="false"
                enterkeyhint="done"
                bq-ref="url"
                bq-on:focus="start"
                bq-on:blur="leave"
                bq-on:keydown|escape|prevent="cancel"
                bq-on:keydown|enter|prevent="apply"
                bq-on:input="clear-problem";
            a.icon-button
                target="_blank"
                rel="noopener noreferrer"
                aria-label="Open link in a new tab"
                title="Open link in a new tab"
                bq-ref="open"
            {
                Icon(Glyph::ExternalLink);
            }
            button.icon-button
                type="button"
                aria-label="Remove link"
                title="Remove link"
                disabled
                bq-ref="remove"
                bq-on:mousedown|prevent
                bq-on:click="unlink"
            {
                Icon(Glyph::Unlink);
            }
        }
    }
}
