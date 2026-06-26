use maud::{Markup, html};

pub enum ButtonElement {
    Button, A, Form
}

pub fn button(content: Markup, element: ButtonElement, on_click: &str) -> Markup {
    match element {
        ButtonElement::Button => html!{
            button onclick={(on_click)} {
                (content)
            }
        },
        ButtonElement::A => html!{
            a href={(on_click)} {
                (content)
            }
        },
        ButtonElement::Form => html! {
            form method="POST" action={(on_click)} {
                button type="submit" { (content) }
            }
        }
    }
}
