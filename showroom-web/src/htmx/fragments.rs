use std::collections::HashMap;
use maud::{Markup, html};

pub fn error(message: &str) -> Markup {
    html! { p { (message) } }
}

pub fn field_errors(fields: &[(&str, Option<&str>)]) -> Markup {
    html! {
        @for (field, error) in fields {
            p.error id=(format!("{}-error", field)) hx-swap-oob="true" {
                @if let Some(err) = error { (err) }
            }
        }
    }
}

pub fn from_errors(errors: validator::ValidationErrors) -> Markup {
    let mut fields: HashMap<String, String> = HashMap::new();
    for (field, field_errors) in errors.field_errors() {
        if let Some(error) = field_errors.first() {
            let message = error.message
                .as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| format!("Invalid {field}"));
            fields.insert(field.to_string(), message);
        }
    }
    html! {
        @for (field, message) in &fields {
            p.error id=(format!("{}-error", field)) hx-swap-oob="true" { (message) }
        }
    }
}
