use maud::{Markup, html};
use crate::views::{context::PageContext, layouts::{ViewContext, base}};

pub fn dashboard_shell(view: ViewContext, ctx: &PageContext, content: Markup) -> Markup {
    let user = ctx.user.as_ref().expect("user is defined");
    let domain = ctx.urls.domain();

    let subtitle = match &ctx.publication {
        Some(publication) => format!("{}.{}", publication.slug, domain),
        None => user.email.clone(),
    };

    base(
        &view,
        html! {
            div.dashboard {
                ul.side-bar {
                    nav {
                        a.logo href=(ctx.urls.base()) {
                            img.logo src="/icons/logo.png" alt="Logo";
                        }

                        @if let Some(publication) = ctx.primary_publication() {
                            @let dashboard = ctx.urls.dashboard(&publication.slug);
                            li {
                                a.home href=(dashboard) { (publication.slug) "." (domain) }
                            }
                            li {
                                a.subscribers href={ (dashboard) "/subscribers" } { "Subscribers" }
                            }
                            li {
                                a.settings href={ (dashboard) "/settings" } { "Settings" }
                            }
                        }
                    }

                    ul {
                        @for publication in ctx.secondary_publications() {
                            li {
                                a.publication href=(ctx.urls.dashboard(&publication.slug)) {
                                    (publication.slug) "." (domain)
                                }
                            }
                        }
                        li {
                            a.account href={ (ctx.urls.app()) "/settings" } { "Account" }
                        }
                    }
                }
                main class=[&view.class] {
                    header {
                        div {
                            h1 { (view.title) }
                            p.subtitle { (subtitle) }
                        }
                    }
                    (content)
                }
            }
        }
    )
}
