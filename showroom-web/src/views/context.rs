use crate::{auth::context::UserContext, models::user, state::Urls};

pub struct ViewerUser {
    pub id: String,
    pub handle: String,
}

pub struct PageOwner {
    pub handle: String,
}

pub struct PageContext {
    pub user: Option<ViewerUser>,
    pub page_owner: Option<PageOwner>,
    pub urls: Urls,
}

impl PageContext {
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }

    pub fn public(ctx: &UserContext, urls: Urls) -> Self {
        Self {
            user: ctx.user_id.as_ref().zip(ctx.handle.as_ref()).map(|(id, handle)| {
                ViewerUser { id: id.clone(), handle: handle.clone() }
            }),
            page_owner: None,
            urls,
        }
    }

    pub fn from_user(user: &user::Model, urls: Urls) -> Self {
        Self {
            user: Some(ViewerUser {
                id: user.id.clone(),
                handle: user.handle.clone(),
            }),
            page_owner: None,
            urls,
        }
    }

    pub fn with_page_owner(mut self, handle: &str) -> Self {
        self.page_owner = Some(PageOwner { handle: handle.to_string() });
        self
    }
}
