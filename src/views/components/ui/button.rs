use maud::{Markup, html};

#[allow(dead_code)]
pub enum ButtonElement {
    Button, A, Form
}

pub fn button(content: Markup, element: ButtonElement, on_click: &str, class: Option<&str>) -> Markup {
    let class = class.map(|c| format!("button {c}")).unwrap_or("button".to_string());

    match element {
        ButtonElement::Button => html!{
            button onclick={(on_click)} class=(class) {
                (content)
            }
        },
        ButtonElement::A => html!{
            a href={(on_click)} class=(class) {
                (content)
            }
        },
        ButtonElement::Form => html! {
            form method="POST" action={(on_click)} class=(class) {
                button type="submit" { (content) }
            }
        }
    }
}
