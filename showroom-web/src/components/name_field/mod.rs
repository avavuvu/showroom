use boutique::html;
use bq_components::component;
use maud::Markup;
use strum::{EnumMessage, IntoEnumIterator};

use crate::models::publication_name::{Article, MAX_LABEL_LEN, NameCase, Noun, Possessive, PublicationName};

#[component]
pub fn name_field(#[positional] name: &PublicationName, case: NameCase, preview_url: &str) -> Markup {
    html! {
        fieldset.name-field hx-post=(preview_url) hx-trigger="input delay:200ms, change" hx-target="#name-preview" {
            legend #name-preview { (name.render(case)) }
            NameCases(case);
            div.name-parts {
                div.input-component {
                    select #name-article name="article" {
                        @for article in Article::iter() {
                            option value=(article.as_ref()) selected[article == name.article] { (article.get_message().unwrap_or_default()) }
                        }
                    }
                }
                div.input-component.name-label {
                    div.shell {
                        input #name-label type="text" name="label" value=(name.label) placeholder="Ava" maxlength=(MAX_LABEL_LEN) autocomplete="off" required;
                    }
                }
                div.input-component {
                    select #name-possessive name="possessive" {
                        @for possessive in Possessive::iter() {
                            option value=(possessive.as_ref()) selected[possessive == name.possessive] { (possessive.get_message().unwrap_or_default()) }
                        }
                    }
                }
                div.input-component {
                    select #name-noun name="noun" {
                        @for noun in Noun::iter() {
                            option value=(noun.as_ref()) selected[noun == name.noun] { (noun.as_ref()) }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn name_cases(#[positional] case: NameCase) -> Markup {
    html! {
        div.name-cases id="name-cases" {

            @for option in NameCase::iter() {
                label.name-case title=[option.get_message()] {
                    input type="radio" name="name_case" value=(option.as_ref()) checked[option == case];
                    span.name-case-example { (option.get_message().unwrap_or_default()) }
                }
            }
        }
    }
}
