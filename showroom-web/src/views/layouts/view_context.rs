pub struct ViewContext {
    pub title: String,
    pub htmx: bool,
    pub alpine: bool,
    pub islands: bool,
    pub scripts: Vec<String>,
    pub metadata: Option<Metadata>,
    pub class: Option<String>,
}

impl ViewContext {
    /// Exact title, no suffix.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            htmx: false,
            alpine: false,
            islands: false,
            scripts: Vec::new(),
            metadata: None,
            class: None,
        }
    }

    /// Appends " – Showroom" to the title.
    pub fn page(title: impl Into<String>) -> Self {
        Self::new(format!("{}", title.into()))
    }

    pub fn htmx(mut self) -> Self {
        self.htmx = true;
        self
    }

    pub fn alpine(mut self) -> Self {
        self.alpine = true;
        self
    }

    pub fn islands(mut self) -> Self {
        self.islands = true;
        self
    }

    pub fn js(mut self, name: impl Into<String>) -> Self {
        self.scripts.push(name.into());
        self
    }

    pub fn seo(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn class(mut self, name: String) -> Self {
        self.class = Some(name);
        self
    }
}

pub enum OgType {
    Website,
    Article,
}

impl OgType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OgType::Website => "website",
            OgType::Article => "article",
        }
    }
}

pub struct Metadata {
    pub description: String,
    pub og_type: OgType,
    pub image: Option<String>,
    pub author: Option<String>,
    pub url: Option<String>,
    pub modified_time: Option<String>,
}

impl Metadata {
    pub fn website(description: &str) -> Self {
        Self {
            description: description.to_string(),
            og_type: OgType::Website,
            image: None,
            author: None,
            url: None,
            modified_time: None,
        }
    }

    pub fn article(description: &str, author: &str, url: &str) -> Self {
        Self {
            description: description.to_string(),
            og_type: OgType::Article,
            author: Some(author.to_string()),
            url: Some(url.to_string()),
            image: None,
            modified_time: None,
        }
    }

    pub fn with_image(mut self, url: &str) -> Self {
        self.image = Some(url.to_string());
        self
    }

    pub fn with_modified_time(mut self, time: &str) -> Self {
        self.modified_time = Some(time.to_string());
        self
    }
}
