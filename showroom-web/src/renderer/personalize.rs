use std::borrow::Cow;

use boutique::html;

use crate::components::NameToken;

pub const NAME_START: &str = "<!--showroom:name-->";
pub const NAME_END: &str = "<!--showroom:end-name-->";
pub const IF_NAME: &str = "<!--showroom:if-name-->";
pub const END_IF_NAME: &str = "<!--showroom:end-if-name-->";
pub const BRACE_VALUE: &str = "{";
const BRACE: &str = "{{brace}}";

pub fn escape(text: &str) -> Cow<'_, str> {
    if text.contains('{') { Cow::Owned(text.replace('{', BRACE)) } else { Cow::Borrowed(text) }
}

pub fn template(html: &str) -> String {
    replace_names(&escape(html), |fallback| format!("{{{{#if name}}}}{{{{name}}}}{{{{else}}}}{fallback}{{{{/if}}}}"))
        .replace(IF_NAME, "{{#if name}}")
        .replace(END_IF_NAME, "{{/if}}")
}

pub fn preview(html: &str, name: Option<&str>) -> String {
    let named = match name {
        Some(name) => replace_names(html, |_| html_escape::encode_text(name).into_owned()),
        None => replace_names(html, |fallback| {
            let fallback = html_escape::decode_html_entities(fallback);
            html! { NameToken(fallback = [Some(fallback.as_ref()).filter(|fallback| !fallback.is_empty())]); }.into_string()
        }),
    };
    named.replace(IF_NAME, "").replace(END_IF_NAME, "")
}

fn replace_names(html: &str, render: impl Fn(&str) -> String) -> String {
    let mut output = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(start) = rest.find(NAME_START) {
        let after = &rest[start + NAME_START.len()..];
        let Some(end) = after.find(NAME_END) else { break };
        output.push_str(&rest[..start]);
        output.push_str(&render(&after[..end]));
        rest = &after[end + NAME_END.len()..];
    }
    output.push_str(rest);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{document::Document, renderer::email::{EmailBlock, render_email}, state::Urls, theme::Theme};

    #[test]
    fn typed_names_become_placeholders_and_their_paragraphs_need_a_name() {
        let content = serde_json::json!({
            "type": "doc",
            "content": [
                { "type": "paragraph", "content": [{ "type": "text", "text": "Hi {{ Name }}," }] },
                { "type": "paragraph", "content": [{ "type": "text", "text": "Code: {{x}}" }] }
            ]
        });
        let document = Document::from_value(&content).unwrap();
        let urls = Urls { site_domain: "showroom.you".to_string(), publication_domain: "room.lc".to_string(), ..Urls::default() };
        let blocks = render_email(&document, Theme::default().email(), &urls);
        let [EmailBlock::Content(html)] = blocks.as_slice() else { panic!("expected one content block") };
        let html = template(html);

        assert!(html.contains("{{#if name}}"), "{html}");
        assert!(html.contains("Hi {{#if name}}{{name}}{{else}}{{/if}},"), "{html}");
        assert!(html.contains("Code: {{brace}}{{brace}}x}}"), "{html}");
    }

    #[test]
    fn a_fallback_keeps_the_paragraph_for_everyone() {
        let html = template(&format!("<p>Hi {NAME_START}friend{NAME_END},</p>"));
        assert_eq!(html, "<p>Hi {{#if name}}{{name}}{{else}}friend{{/if}},</p>");
        let shown = preview(&format!("<p>Hi {NAME_START}friend{NAME_END},</p>"), None);
        assert!(shown.contains("name-token-fallback"), "{shown}");
    }

    #[test]
    fn user_braces_cannot_open_handlebars_tags() {
        let html = format!("<p>{IF_NAME}Hi {NAME_START}{NAME_END}, see {{{{unsubscribe_url}}}}{END_IF_NAME}</p>");
        assert_eq!(template(&html), "<p>{{#if name}}Hi {{#if name}}{{name}}{{else}}{{/if}}, see {{brace}}{{brace}}unsubscribe_url}}{{/if}}</p>");
    }
}
