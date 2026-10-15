use boutique::html;
use bq_components::{Attrs, component};
use maud::Markup;

use crate::theme::Color;

#[component]
pub fn color_input(
    #[builder(start_fn)] name: &str,
    label: &str,
    current_color: &Color,
    #[builder(default)] presets: &[Color],
    #[builder(default)] attrs: Attrs,
) -> Markup {
    html! {
        details.color-input bq-setup style={ "--color: " (current_color) } {
            summary {
                span.swatch {}
                span.label { (label) }
                span.hex bq-ref="hex" { (current_color) }
            }

            div.options {
                @for color in presets {
                    button.color-button
                        type="button"
                        bq-ref="preset"
                        bq-on:click="pick"
                        value=(color)
                        title=(color)
                        aria-label=(color)
                        aria-pressed=(color == current_color)
                        style={ "--color: " (color) } {}
                }

                label.custom {
                    input type="color" name=(name) value=(current_color) bq-ref="custom-color" bq-on:input="change" (..attrs);
                }
            }
        }
    }
}
