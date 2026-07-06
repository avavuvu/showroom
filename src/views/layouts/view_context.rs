pub struct ViewContext {
    pub title: String,
    // pub description: String,
    pub islands: bool,
    pub js: bool,
}

impl Default for ViewContext {
    fn default() -> Self {
        Self {
            title: "Showroom".into(),
            // description: String::default(),
            js: false,
            islands: false,
        }
    }
}

impl ViewContext {
    pub fn metadata(title: &str) -> Self {
        Self {
            title: format!("{} – Showroom", title),
            js: false,
            islands: false
        }
    }
}
