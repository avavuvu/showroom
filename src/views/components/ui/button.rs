use maud::{Markup, html};

#[allow(dead_code)]
pub enum ButtonElement {
    Button, A, Form
}

pub fn button(content: Markup, element: ButtonElement, on_click: &str) -> Markup {
    match element {
        ButtonElement::Button => html!{
            button.btn onclick={(on_click)} {
                (content)
            }
        },
        ButtonElement::A => html!{
            a.btn href={(on_click)} {
                (content)
            }
        },
        ButtonElement::Form => html! {
            form.btn method="POST" action={(on_click)} {
                button type="submit" { (content) }
            }
        }
    }
}
