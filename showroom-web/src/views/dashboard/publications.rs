use boutique::html;
use maud::Markup;
use crate::components::{Button, Input, Kind, Textarea};
use crate::views::context::PageContext;
use crate::views::layouts::{page, dashboard_shell};

pub fn new_form(ctx: &PageContext, slug: &str, name: &str, error: Option<&str>) -> Markup {
    let domain = format!(".{}", ctx.urls.domain());

    dashboard_shell(
        page("New publication").class("settings".into()),
        ctx,
        html! {
            section.settings-section {
                form.settings-form method="POST" action={ (ctx.urls.app()) "/new" } {
                    @if let Some(error) = error {
                        p.error { (error) }
                    }
                    Input("name", label = "Name") value=(name) placeholder="My newsletter" required;
                    Input(
                        "slug",
                        label = "Address",
                        suffix = &domain,
                        hint = "Lowercase letters, numbers and hyphens. This cannot be changed later.",
                    )
                        value=(slug)
                        placeholder="my-newsletter"
                        pattern="[a-z0-9-]{3,40}"
                        minlength="3"
                        maxlength="40"
                        required;
                    Button(submit = true) .primary { "Create publication" }
                }
            }
        }
    )
}

pub fn settings(ctx: &PageContext, error: Option<&str>) -> Markup {
    let publication = ctx.publication();
    let dashboard_url = ctx.dashboard_url();
    let address = format!("{}.{}", publication.slug, ctx.urls.domain());
    let theme = publication.theme();
    let (ink, paper, brand) = (theme.ink.to_string(), theme.paper.to_string(), theme.brand.to_string());

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
                    Input("name", label = "Name") value=(publication.name) required;
                    Textarea("description", label = "Description", value = publication.description.as_deref().unwrap_or("")) rows="3";
                    Input("address", label = "Address") value=(address) disabled;
                    Button(submit = true) .primary { "Save" }
                }
            }

            section.settings-section {
                h2 { "Style" }
                form.settings-form method="POST" action={ (dashboard_url) "/settings/style" } {
                    Input("ink", kind = Kind::Color, label = "Ink") value=(ink);
                    Input("paper", kind = Kind::Color, label = "Paper") value=(paper);
                    Input("brand", kind = Kind::Color, label = "Brand") value=(brand);
                    Button(submit = true) .primary { "Save" }
                }
            }

            @if !publication.is_default {
                section.settings-section.danger-zone {
                    h2 { "Delete publication" }
                    p.hint { "This removes the publication, its newsletters and its subscribers. This cannot be undone." }
                    form method="POST" action={ (dashboard_url) "/delete" }
                        onsubmit="return confirm('Delete this publication and everything in it?')" {
                        Button(submit = true) .danger { "Delete " (publication.name) }
                    }
                }
            }
        }
    )
}
