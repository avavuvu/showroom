use boutique::html;
use bq_components::component;
use maud::Markup;

use crate::theme::{Color, Trim};

const DEFAULT_TRIM_COLORS: &[Color] = &[
    Color::hex(0x3B2A20),
    Color::hex(0x8B5A2B),
    Color::hex(0x222222),
    Color::hex(0xF6F1E7),
    Color::hex(0x1B2A4A),
    Color::hex(0x316F2F),
    Color::hex(0xC0392B),
];

fn trim_label(trim: Trim) -> String {
    match trim {
        Trim::Wood => "Wood".to_string(),
        Trim::Plain => "None".to_string(),
        Trim::Color(color) => color.to_string(),
    }
}

fn trim_swatch(trim: Trim) -> String {
    match trim {
        Trim::Wood => "var(--trim-wood)".to_string(),
        Trim::Plain => "var(--color-primary)".to_string(),
        Trim::Color(color) => color.to_string(),
    }
}

#[component(setup = "trim-input")]
pub fn trim_input(#[positional] current: Trim, brand: &Color) -> Markup {
    let picker_color = current.color().unwrap_or(*brand);
    html! {
        details.color-input.trim-input bq-setup style={ "--trim-swatch: " (trim_swatch(current)) } {
            summary {
                span.swatch {}
                span.label { "Trim" }
                span.hex bq-ref="hex" { (trim_label(current)) }
            }

            div.options {
                button.color-button.trim-wood
                    type="button"
                    bq-ref="preset"
                    bq-on:click="pick"
                    value="wood"
                    title="Wood"
                    aria-label="Wood"
                    aria-pressed=(current == Trim::Wood) {}
                button.color-button.trim-none
                    type="button"
                    bq-ref="preset"
                    bq-on:click="pick"
                    value="none"
                    title="None"
                    aria-label="None"
                    aria-pressed=(current == Trim::Plain) {}
                @for color in DEFAULT_TRIM_COLORS {
                    button.color-button
                        type="button"
                        bq-ref="preset"
                        bq-on:click="pick"
                        value=(color)
                        title=(color)
                        aria-label=(color)
                        aria-pressed=(current == Trim::Color(*color))
                        style={ "--color: " (color) } {}
                }

                label.custom {
                    input type="color" value=(picker_color) aria-label="Trim color" bq-ref="custom-color" bq-on:input="change";
                }
            }

            input type="hidden" name="trim" value=(current.key()) bq-ref="value";
        }
    }
}
