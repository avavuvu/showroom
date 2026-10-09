use boutique::html;
use bq_components::{Attrs, component};
use maud::Markup;

#[component]
pub fn article_header(
    date: &str,
    title: &str,
    #[builder(required)] subtitle: Option<&str>,
    name: &str,
    href: &str,
    picture: &str,
    #[builder(default)] attrs: Attrs,
) -> Markup {
    html! {
        div.article-header (..attrs) {
            p.date { (date) }
            h1.title { (title) }
            @if let Some(subtitle) = subtitle {
                p.subtitle { (subtitle) }
            }
            p.handle {
                a href=(href) {
                    img.publication-picture src=(picture) alt="";
                    (name)
                }
            }
        }
    }
}
