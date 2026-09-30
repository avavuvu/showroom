use bq_components::{component, setup};
use maud::{Markup, html};

setup!(NewsletterEditor);

#[component]
pub fn editor(#[builder(start_fn)] newsletter_id: &str) -> Markup {
    let props = serde_json::json!({ "newsletterId": newsletter_id }).to_string();

    html! {
        sr-editor data-props=(props) bq-setup=(NewsletterEditor) {}
    }
}
