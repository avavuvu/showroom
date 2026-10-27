use boutique::html;
use bq_components::component;
use chrono::{DateTime, FixedOffset};
use maud::Markup;

#[component(setup = "local-time")]
pub fn local_time(#[positional] value: &DateTime<FixedOffset>) -> Markup {
    html! {
        time datetime=(value.to_rfc3339()) bq-setup { (value.format("%B %-d, %-I:%M %p")) }
    }
}
