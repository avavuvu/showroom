use maud::{Markup, html};

use crate::{state::Urls, views::components::ui::*};

pub fn header(is_authenticated: bool, urls: &Urls) -> Markup {
    html! {
        header class="justify-between flex" {
            div class="px-2 flex items-center" {
                a class="p-4" {
                    img class="h-24" src="logoWordmark" alt="Showroom";
                }
            }
            div class="flex justify-end items-end" {
                @if is_authenticated {
                    (button(
                        html!{"Log out"},
                        ButtonElement::Form,
                        &format!("{}/logout", urls.base()))
                    )
                    (button(
                        html!{"Dashboard"},
                        ButtonElement::A,
                        &urls.app())
                    )
                }
                @else {
                    (button(
                        html!{"Get started"},
                        ButtonElement::A,
                        &format!("{}/signup", urls.base()))
                    )
                    (button(
                        html!{"Login"},
                        ButtonElement::A,
                        &format!("{}/login", urls.base()))
                    )
                }
            }
        }
    }
}
