use maud::{Markup, html};
use crate::{
    models::subscriber::Model as Subscriber,
    views::{context::PageContext, layouts::{ViewContext, base}},
};

pub fn index(ctx: &PageContext, subscribers: &[Subscriber]) -> Markup {
    base(ViewContext::page("Subscribers").alpine().htmx(), html! {
        h1 { "Subscribers" }
        p id="subscriber-count" { (subscribers.len()) " total" }

        table {
            thead {
                tr {
                    th { "Email" }
                    th { "Name" }
                    th { "Subscribed" }
                }
            }
            (subscriber_tbody(subscribers))
        }

        section {
            h2 { "Import from CSV" }
            p {
                "Required column: " code { "email" }
                ". Optional: " code { "name" } ", " code { "created_at" } " or " code { "subscribed_at" } "."
            }
            div id="import-status" {}
            form
                hx-post="/subscribers/import"
                hx-target="#subscribers-tbody"
                hx-swap="outerHTML"
                hx-encoding="multipart/form-data" {
                input type="file" name="file" accept=".csv" required;
                button type="submit" { "Import" }
            }
        }
    })
}

fn subscriber_tbody(subscribers: &[Subscriber]) -> Markup {
    html! {
        tbody id="subscribers-tbody" {
            @for sub in subscribers {
                tr {
                    td { (sub.email) }
                    td { (sub.name.as_deref().unwrap_or("—")) }
                    td { (sub.created_at.format("%Y-%m-%d")) }
                }
            }
        }
    }
}

pub fn import_result(subscribers: &[Subscriber], skipped: usize) -> Markup {
    html! {
        (subscriber_tbody(subscribers))
        p id="subscriber-count" hx-swap-oob="true" { (subscribers.len()) " total" }
        div id="import-status" hx-swap-oob="true" {
            @if skipped > 0 { p { "Skipped " strong { (skipped) } " invalid rows." } }
        }
    }
}
