use std::collections::HashMap;
use maud::{Markup, html};

pub fn error(message: &str) -> Markup {
    html! { p { (message) } }
}

pub fn field_errors(fields: &[(&str, Option<&str>)]) -> Markup {
    html! {
        @for (field, error) in fields {
            p id=(format!("{}-error", field)) hx-swap-oob="true" {
                @if let Some(err) = error { (err) }
            }
        }
    }
}

pub fn from_report(report: garde::Report) -> Markup {
    let mut fields: HashMap<String, String> = HashMap::new();
    for (path, error) in report.iter() {
        fields.entry(path.to_string()).or_insert_with(|| error.to_string());
    }
    html! {
        @for (field, message) in &fields {
            p id=(format!("{}-error", field)) hx-swap-oob="true" { (message) }
        }
    }
}
