use boutique::html;
use bq_components::component;
use maud::Markup;

use crate::components::icon::{Glyph, Icon};

#[component(setup = "editor-link-bar")]
pub fn link_bar() -> Markup {
    html! {
        div.link-bar.is-idle bq-setup {
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
            input.link-input.link-fallback
                type="text"
                name="fallback"
                placeholder="Friend"
                aria-label="Fallback for subscribers without a name"
                aria-describedby="name-fallback-hint"
                autocomplete="off"
                enterkeyhint="done"
                maxlength="40"
                bq-ref="fallback"
                bq-on:focus="start-name"
                bq-on:blur="leave"
                bq-on:input="change-fallback"
                bq-on:keydown|escape|prevent="cancel-name"
                bq-on:keydown|enter|prevent="finish-name";
            span.link-hint id="name-fallback-hint" {
                "Shown when a subscriber didn't give a name"
            }
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
