use boutique::html;
use bq_components::Button;
use crate::components::{Wordmark, dropdown::Dropdown, icon::{Glyph, Icon}};
use maud::{Markup, PreEscaped};
use crate::views::{context::PageContext, layouts::{Head, base}};

pub fn dashboard_shell(view: Head, ctx: &PageContext, content: Markup) -> Markup {
    let user = ctx.user.as_ref().expect("user is defined");
    let domain = ctx.urls.domain();

    let subtitle = match &ctx.publication {
        Some(publication) => format!("{}.{}", publication.slug, domain),
        None => user.email.clone(),
    };

    let class = view.class.as_deref().unwrap_or("");
    let current = |item: &str| -> Option<&'static str> {
        let active = match item {
            "home" => class == "overview",
            "subscribers" => class == "subscribers",
            "settings" => class == "settings" && ctx.publication.is_some(),
            "account" => class == "settings" && ctx.publication.is_none(),
            _ => false,
        };
        active.then_some("page")
    };
    let others: Vec<_> = ctx.secondary_publications().collect();

    base(
        &view,
        html! {
            @if let Some(publication) = &ctx.publication {
                style id="theme" { (PreEscaped(publication.theme().css())) }
            }
            div.dashboard {
                nav.side-bar {
                    div.side-bar-top {
                        a.logo href=(ctx.urls.base()) {
                            img.mark src="/icons/logo-sm.webp" alt="";
                            Wordmark;
                        }
                        div.side-bar-icons {
                            @if let Some(publication) = ctx.primary_publication() {
                                @let dashboard = ctx.urls.dashboard(&publication.slug);
                                a.icon-link href=(dashboard) aria-label=(publication.name) title=(publication.name) aria-current=[current("home")] {
                                    Icon(Glyph::House);
                                }
                                a.icon-link href={ (dashboard) "/subscribers" } aria-label="Subscribers" title="Subscribers" aria-current=[current("subscribers")] {
                                    Icon(Glyph::Users);
                                }
                                a.icon-link href={ (dashboard) "/settings" } aria-label="Settings" title="Settings" aria-current=[current("settings")] {
                                    Icon(Glyph::Settings);
                                }
                            }
                            @if !others.is_empty() {
                                details.publications-menu bq-setup=(Dropdown) {
                                    summary.icon-link aria-label="Other publications" title="Other publications" {
                                        Icon(Glyph::Layers);
                                    }
                                    ul.publications-list {
                                    @for publication in &others {
                                        li {
                                            a href=(ctx.urls.dashboard(&publication.slug)) {
                                                (publication.name)
                                                span.picture-box {
                                                    img src=(publication.picture_url(&ctx.urls)) alt="";
                                                }
                                            }
                                        }
                                    }
                                    }
                                }
                            }
                            a.icon-link href={ (ctx.urls.app()) "/settings" } aria-label="Account" title="Account" aria-current=[current("account")] {
                                Icon(Glyph::CircleUser);
                            }
                        }
                    }

                    div.side-bar-menu {
                    ul {
                        @if let Some(publication) = ctx.primary_publication() {
                            @let dashboard = ctx.urls.dashboard(&publication.slug);
                            li {
                                Button(href = &dashboard) .home .ghost {
                                    (publication.name)
                                    span.picture-box {
                                        img src=(publication.picture_url(&ctx.urls)) alt="";
                                    }
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
                        @for publication in &others {
                            li {
                                Button(href = &ctx.urls.dashboard(&publication.slug)) .publication .ghost {
                                    (publication.name)
                                    span.picture-box {
                                        img src=(publication.picture_url(&ctx.urls)) alt="";
                                    }
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
