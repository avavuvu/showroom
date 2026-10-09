use boutique::html;
use maud::Markup;

pub const DEFAULT: &str = "Hi {{name}},";
pub const NAME: &str = "{{name}}";
pub const MAX_LEN: usize = 200;

pub fn normalize(template: &str) -> Result<String, &'static str> {
    let mut normalized = String::with_capacity(template.len());
    let mut rest = template;

    while let Some(start) = rest.find("{{") {
        normalized.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let end = after.find("}}").ok_or("Each {{ needs a closing }}.")?;
        if !after[..end].trim().eq_ignore_ascii_case("name") {
            return Err("Only {{name}} is supported.");
        }
        normalized.push_str(NAME);
        rest = &after[end + 2..];
    }

    normalized.push_str(rest);
    Ok(normalized)
}

pub fn for_subscriber(template: &str, name: Option<&str>) -> Option<String> {
    let name = name.map(str::trim).filter(|name| !name.is_empty());
    match name {
        _ if template.is_empty() => None,
        Some(name) => Some(template.replace(NAME, name)),
        None if template.contains(NAME) => None,
        None => Some(template.to_string()),
    }
}

pub fn preview(template: &str) -> Option<Markup> {
    if template.is_empty() {
        return None;
    }

    Some(html! {
        @for (index, text) in template.split(NAME).enumerate() {
            @if index > 0 {
                span.handlebars { "name" }
            }
            (text)
        }
    })
}
