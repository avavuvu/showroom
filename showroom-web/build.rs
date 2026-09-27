use std::{env, path::Path};

const MANIFEST: &str = "public/build/.vite/manifest.json";

fn main() {
    println!("cargo:rerun-if-changed={MANIFEST}");
    if env::var("PROFILE").as_deref() == Ok("release") && !Path::new(MANIFEST).exists() {
        panic!("{MANIFEST} not found: run `npm run build` before a release build");
    }
}
