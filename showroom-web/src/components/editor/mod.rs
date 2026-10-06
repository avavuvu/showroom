use boutique::html;
use bq_components::{component, setup};
use maud::Markup;

setup!(NewsletterEditor);

#[component]
pub fn editor(#[builder(start_fn)] newsletter_id: &str) -> Markup {
    let props = serde_json::json!({ "newsletterId": newsletter_id }).to_string();

    html! {
        div.newsletter-editor data-props=(props) bq-setup=(NewsletterEditor) {}
    }
}
