use maud::{Markup, html};
use crate::views::{context::PageContext, layouts::{ViewContext, base}};

pub fn dashboard_shell(view: ViewContext, ctx: &PageContext, content: Markup) -> Markup {
    base(
        &view,
        html! {
            div.dashboard {
                ul.side-bar {
                    nav {
                        a.logo href=(ctx.urls.base()) {
                            img.logo src="/icons/logo.png" alt="Logo";
                        }
                        li {
                            a.posts href="/" { "Posts" }
                        }
                        li {
                            a.subscribers href="/subscribers" { "Subscribers" }
                        }
                        li {
                            a.settings href="/style" { "Style" }
                        }
                    }

                    ul {
                        li {
                            a.settings href="/settings" { "Settings" }
                        }
                    }
                }
                main class=[&view.class] {
                    header {
                        div {
                            h1 { (view.title) }
                            p.subtitle { (ctx.user.as_ref().expect("user is defined").handle) "." (ctx.urls.domain()) }
                        }
                    }
                    (content)
                }
            }
        }
    )
}
