use crate::{auth::context::UserContext, models::{publication, user}, state::Urls};

pub struct ViewerUser {
    pub id: String,
    pub email: String,
}

pub struct PageContext {
    pub user: Option<ViewerUser>,
    pub publication: Option<publication::Model>,
    pub publications: Vec<publication::Model>,
    pub urls: Urls,
}

impl PageContext {
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }

    pub fn public(ctx: &UserContext, urls: Urls) -> Self {
        Self {
            user: ctx.user_id.as_ref().zip(ctx.email.as_ref()).map(|(id, email)| {
                ViewerUser { id: id.clone(), email: email.clone() }
            }),
            publication: None,
            publications: Vec::new(),
            urls,
        }
    }

    pub fn from_user(user: &user::Model, urls: Urls) -> Self {
        Self {
            user: Some(ViewerUser {
                id: user.id.clone(),
                email: user.email.clone(),
            }),
            publication: None,
            publications: Vec::new(),
            urls,
        }
    }

    pub fn with_publication(mut self, publication: publication::Model) -> Self {
        self.publication = Some(publication);
        self
    }

    pub fn with_publications(mut self, publications: Vec<publication::Model>) -> Self {
        self.publications = publications;
        self
    }

    pub fn publication(&self) -> &publication::Model {
        self.publication.as_ref().expect("view requires a publication")
    }

    pub fn publication_url(&self) -> String {
        self.urls.publication(&self.publication().slug)
    }

    pub fn dashboard_url(&self) -> String {
        self.urls.dashboard(&self.publication().slug)
    }

    pub fn primary_publication(&self) -> Option<&publication::Model> {
        self.publication.as_ref().or_else(|| self.publications.iter().find(|p| p.is_default))
    }

    pub fn secondary_publications(&self) -> impl Iterator<Item = &publication::Model> {
        let primary_id = self.primary_publication().map(|p| p.id.clone());
        self.publications.iter().filter(move |p| Some(&p.id) != primary_id.as_ref())
    }
}
