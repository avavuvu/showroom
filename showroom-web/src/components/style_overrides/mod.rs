use boutique::html;
use bq_components::{Attrs, component, setup};
use maud::Markup;

use crate::theme::{Color, Theme};

setup!(ColorOverride);

struct Row {
    name: &'static str,
    label: &'static str,
    color: Color,
    custom: bool,
}

#[component]
pub fn style_overrides(#[builder(start_fn)] theme: &Theme, #[builder(default)] attrs: Attrs) -> Markup {
    let rows = [
        Row {
            name: "link",
            label: "Links",
            color: theme.link(),
            custom: theme.overrides.link.is_some(),
        },
        Row {
            name: "muted",
            label: "Muted",
            color: theme.muted(),
            custom: theme.overrides.muted.is_some(),
        },
    ];

    html! {
        details.color-input.style-overrides (..attrs) {
            summary {
                span.label { "More styling" }
            }

            @for row in &rows {
                div.options.override bq-setup=(ColorOverride) style={ "--color: var(--" (row.name) ")" } {
                    span.swatch {}
                    span.label { (row.label) }
                    span.hex bq-ref="hex" {
                        @if row.custom { (row.color) }
                    }

                    button.button.ghost.small.reset
                        type="button"
                        bq-ref="reset"
                        aria-label={ "Reset " (row.label.to_lowercase()) " color" }
                        title="Reset"
                        hidden[!row.custom] {
                        svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" {
                            path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" {}
                            path d="M3 3v5h5" {}
                        }
                    }

                    label.custom {
                        input type="color" name=(row.name) value=(row.color) bq-ref="picker" aria-label={ (row.label) " color" };
                        "Override"
                    }

                    input type="checkbox" name={ (row.name) "_custom" } value="on" checked[row.custom] hidden bq-ref="toggle";
                }
            }
        }
    }
}
