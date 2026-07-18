use maud::{Markup, html};

pub fn input(id: &str, name: &str, kind: &str, autocomplete: &str, placeholder: &str, is_required: bool) -> Markup {
    html! {
        div.input-component {
            div.shell {
                input
                    id=(id)
                    name=(name)
                    type=(kind)s
                    autocomplete=(autocomplete)
                    placeholder=(placeholder)
                    required?[is_required];
            }

            p.error id=(format!("{}-error", name)) {}
        }
    }
}
