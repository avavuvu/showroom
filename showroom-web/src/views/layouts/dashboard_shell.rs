use boutique::html;
use bq_components::Button;
use maud::{Markup, PreEscaped};
use crate::views::{context::PageContext, layouts::{Head, base}};

pub fn dashboard_shell(view: Head, ctx: &PageContext, content: Markup) -> Markup {
    let user = ctx.user.as_ref().expect("user is defined");
    let domain = ctx.urls.domain();

    let subtitle = match &ctx.publication {
        Some(publication) => format!("{}.{}", publication.slug, domain),
        None => user.email.clone(),
    };

    base(
        &view,
        html! {
            @if let Some(publication) = &ctx.publication {
                style id="theme" { (PreEscaped(publication.theme().css())) }
            }
            div.dashboard {
                nav.side-bar {
                    ul {
                        a.logo href=(ctx.urls.base()) {
                            img.logo src="/icons/logo.png" alt="Logo";
                        }

                        @if let Some(publication) = ctx.primary_publication() {
                            @let dashboard = ctx.urls.dashboard(&publication.slug);
                            li {
                                Button(href = &dashboard) .home .ghost {
                                    (publication.name)
                                }
                            }
                            li {
                                Button(href = &format!("{dashboard}/subscribers")) .subscribers .ghost {
                                    "Subscribers"
                                }
                            }
                            li {
                                Button(href = &format!("{dashboard}/settings")) .settings .ghost {
                                    "Settings"
                                }
                            }
                        }
                    }

                    ul {
                        @for publication in ctx.secondary_publications() {
                            li {
                                Button(href = &ctx.urls.dashboard(&publication.slug)) .publication .ghost {
                                    (publication.name)
                                }
                            }
                        }
                        li {
                            Button(href = &format!("{}/settings", ctx.urls.app())) .account .ghost {
                                "Account"
                            }
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
