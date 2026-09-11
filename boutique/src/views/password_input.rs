use maud::{Markup, html};

pub fn password_input(name: &str, label: &str) -> Markup {
    html! {
        div.input-component {
            label for=(name) { (label) }
            div
                .shell
                .password
                x-data="{ show: false }" {
                input.password-input
                    id=(name)
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
            p.error id=(format!("{}-error", name)) {}
        }
    }
}
