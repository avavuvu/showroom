use boutique::{assets::manifest, views::Head};

pub fn page(title: impl Into<String>) -> Head {
    let head = Head::new(title).favicon("/favicon.ico").entry("site");
    match manifest::url("islands") {
        Some(src) => head.islands_entry(src),
        None => head,
    }
}
