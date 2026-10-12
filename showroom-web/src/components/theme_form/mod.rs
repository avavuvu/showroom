mod color_input;
mod style_overrides;

use boutique::html;
use bq_components::{component, setup};
use maud::Markup;

use crate::components::{Button, DirtyForm};
use crate::theme::{Color, Font, Layout, Theme};
use color_input::ColorInput;
use style_overrides::StyleOverrides;

setup!(ThemePreview);

const DEFAULT_INK_COLORS: &[Color] = &[
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

const DEFAULT_PAPER_COLORS: &[Color] = &[
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

const DEFAULT_BRAND_COLORS: &[Color] = &[
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
pub fn theme_form(#[builder(start_fn)] action: &str, theme: &Theme) -> Markup {
    html! {
        DirtyForm(action) .settings-form bq-setup=(ThemePreview) bq-on:input|capture="theme-preview.schedule" {
            ColorInput("ink", label = "Ink", current_color = &theme.ink, presets = DEFAULT_INK_COLORS);
            ColorInput("paper", label = "Paper", current_color = &theme.paper, presets = DEFAULT_PAPER_COLORS);
            ColorInput("brand", label = "Brand", current_color = &theme.brand, presets = DEFAULT_BRAND_COLORS);
            StyleOverrides(theme);
            fieldset.font-fields {
                legend { "Fonts" }
                (font_list("font_title", "Title", theme.fonts.title))
                (font_list("font_body", "Body", theme.fonts.body))
            }
            (layout_list(theme.layout))
            Button(submit = true) .primary { "Save" }
        }
    }
}

fn font_list(name: &str, label: &str, current: Font) -> Markup {
    html! {
        fieldset.option-list {
            legend { (label) }
            @for font in Font::ALL {
                label.option style={ "font-family: " (font.stack()) } {
                    input type="radio" name=(name) value=(font.key()) checked[font == current];
                    span { (font.label()) }
                }
            }
        }
    }
}

fn layout_list(current: Layout) -> Markup {
    html! {
        fieldset.option-list.layout-fields {
            legend { "Layout" }
            @for layout in Layout::ALL {
                label.option {
                    input type="radio" name="layout" value=(layout.key()) checked[layout == current];
                    span { (layout.label()) }
                }
            }
        }
    }
}
