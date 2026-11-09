pub mod email;

use boutique::html;
use maud::Markup;

use crate::views::{PageContext, layouts::notice_page};

pub enum Unavailable {
    Expired,
    Removed,
    Disabled,
}

pub fn unavailable(ctx: &PageContext, reason: Unavailable) -> Markup {
    let (title, message) = match reason {
        Unavailable::Expired => ("This link has expired", "Leave your comment again and we'll send you a new link."),
        Unavailable::Removed => ("This comment was removed", "The author of this newsletter removed it."),
        Unavailable::Disabled => ("Comments are turned off", "This publication isn't taking comments at the moment."),
    };

    notice_page(ctx, title, html! {
        p { (message) }
        a.link href=(ctx.publication_url()) { "Back to " (ctx.publication().name) }
    })
}
