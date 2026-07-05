use maud::{Markup, html};

pub fn handle_input() -> Markup {
    html! {
        div.input-component {
            div.shell.handle {
                span {
                    "@"
                }
                input.handle-input
                    id="handle"
                    name="handle"
                    autocomplete="username"
                    placeholder="yourhandle"
                    required;
            }
            p.error id="handle-error" {}
        }
    }
}
