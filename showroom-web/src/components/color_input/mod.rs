use boutique::html;
use bq_components::{Attrs, component, setup};
use maud::Markup;

use crate::theme::Color;

setup!(ColorInput);
setup!(pub ThemePreview);

pub const DEFAULT_INK_COLORS: &[Color] = &[
    Color::hex(0x000000),
    Color::hex(0x222222),
    Color::hex(0x1B2A4A),
    Color::hex(0x3B2A20),
    Color::hex(0x5778CF),
    Color::hex(0x9FA8A8),
    Color::hex(0xE3EBDC),
    Color::hex(0x366c94),
    Color::hex(0xd6d6d6),
];

pub const DEFAULT_PAPER_COLORS: &[Color] = &[
    Color::hex(0xFFFFFF),
    Color::hex(0xF6F1E7),
    Color::hex(0xEEEEEE),
    Color::hex(0x111111),
    Color::hex(0xD6FFEE),
    Color::hex(0xFFF2DB),
    Color::hex(0x60D484),
    Color::hex(0xfaf3ea),
    Color::hex(0x315178),
];

pub const DEFAULT_BRAND_COLORS: &[Color] = &[
    Color::hex(0x92CA3A),
    Color::hex(0x0079FE),
    Color::hex(0x0000FF),
    Color::hex(0xF66F42),
    Color::hex(0xFFC700),
    Color::hex(0xF66FD3),
    Color::hex(0x316F2F),
    Color::hex(0x759D86),
    Color::hex(0x366c94),
    Color::hex(0x315178),
];

#[component]
pub fn color_input(
    #[builder(start_fn)] name: &str,
    label: &str,
    current_color: &Color,
    #[builder(default)] presets: &[Color],
    #[builder(default)] attrs: Attrs,
) -> Markup {
    html! {
        details.color-input bq-setup=(ColorInput) style={ "--color: " (current_color) } {
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
                        value=(color)
                        title=(color)
                        aria-label=(color)
                        aria-pressed=(color == current_color)
                        style={ "--color: " (color) } {}
                }

                label.custom {
                    input type="color" name=(name) value=(current_color) bq-ref="custom-color" (..attrs);
                }
            }
        }
    }
}
