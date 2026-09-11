use maud::{Markup, html};
use crate::views::context::PageContext;
use crate::views::layouts::{page, dashboard_shell};

pub fn new_form(ctx: &PageContext, slug: &str, name: &str, error: Option<&str>) -> Markup {
    dashboard_shell(
        page("New publication").class("settings".into()),
        ctx,
        html! {
            section.settings-section {
                form.settings-form method="POST" action={ (ctx.urls.app()) "/new" } {
                    @if let Some(error) = error {
                        p.error { (error) }
                    }
                    label {
                        "Name"
                        input type="text" name="name" value=(name) placeholder="My newsletter" required;
                    }
                    label {
                        "Address"
                        div.address-input {
                            input type="text" name="slug" value=(slug) placeholder="my-newsletter"
                                pattern="[a-z0-9-]{3,40}" minlength="3" maxlength="40" required;
                            span { "." (ctx.urls.domain()) }
                        }
                        p.hint { "Lowercase letters, numbers and hyphens. This cannot be changed later." }
                    }
                    button.button.button-primary type="submit" { "Create publication" }
                }
            }
        }
    )
}

pub fn settings(ctx: &PageContext, error: Option<&str>) -> Markup {
    let publication = ctx.publication();
    let dashboard_url = ctx.dashboard_url();

    dashboard_shell(
        page("Settings").class("settings".into()),
        ctx,
        html! {
            section.settings-section {
                h2 { "Details" }
                form.settings-form method="POST" action={ (dashboard_url) "/settings" } {
                    @if let Some(error) = error {
                        p.error { (error) }
                    }
                    label {
                        "Name"
                        input type="text" name="name" value=(publication.name) required;
                    }
                    label {
                        "Description"
                        textarea name="description" rows="3" { (publication.description.as_deref().unwrap_or("")) }
                    }
                    label {
                        "Address"
                        input type="text" value={ (publication.slug) "." (ctx.urls.domain()) } disabled;
                    }
                    button.button.button-primary type="submit" { "Save" }
                }
            }

            section.settings-section {
                h2 { "Style" }
                p.hint { "The theme editor is not ready yet." }
            }

            @if !publication.is_default {
                section.settings-section.danger-zone {
                    h2 { "Delete publication" }
                    p.hint { "This removes the publication, its newsletters and its subscribers. This cannot be undone." }
                    form method="POST" action={ (dashboard_url) "/delete" }
                        onsubmit="return confirm('Delete this publication and everything in it?')" {
                        button.button.button-danger type="submit" { "Delete " (publication.name) }
                    }
                }
            }
        }
    )
}
