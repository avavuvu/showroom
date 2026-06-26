use maud::{Markup, html};

pub fn input(id: &str, name: &str, kind: &str, autocomplete: &str, placeholder: &str, is_required: bool) -> Markup {
    html! {
        div {
            input
                id=(id)
                name=(name)
                type=(kind)
                autocomplete=(autocomplete)
                placeholder=(placeholder)
                required?[is_required];
            p id=(format!("{}-error", name)) {}
        }
    }
}
