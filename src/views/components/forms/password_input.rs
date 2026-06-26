use maud::{Markup, html};

pub fn password_input(name: &str, label: &str) -> Markup {
    html! {
        div {
            label { (label) }
            div x-data="{ show: false }" {
                input
                    name=(name)
                    x-bind:type="show ? 'text' : 'password'"
                    autocomplete="current-password"
                    required;
                button
                    type="button"
                    tabindex="-1"
                    x-on:click="show = !show"
                    x-text="show ? 'Hide' : 'Show'"
                    {}
            }
            p id=(format!("{}-error", name)) {}
        }
    }
}
