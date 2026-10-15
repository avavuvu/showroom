use boutique::views::Head;

pub fn page(title: impl Into<String>) -> Head {
    Head::new(title).favicon("/favicon.ico").entry("site")
}

pub fn installable(head: Head) -> Head {
    head.manifest("/manifest.webmanifest").theme_color("#ffffff")
}
