use maud::{Markup, html};
use crate::{state::Urls, views::components::layout::header::header};

use super::layouts::base;

pub fn index(is_authenticated: bool, urls: &Urls) -> Markup {
    base(html! {
        (header(is_authenticated, urls))

        div {

        }
    })
}
