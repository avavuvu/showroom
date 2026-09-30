use boutique::views::Head;

pub fn page(title: impl Into<String>) -> Head {
    Head::new(title).favicon("/favicon.ico").entry("site")
}
