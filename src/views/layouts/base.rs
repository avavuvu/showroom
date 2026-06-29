use maud::{DOCTYPE, Markup, html};

pub fn base(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Showroom" }
                link rel="stylesheet" href="/assets/styles.css";
                link rel="stylesheet" href="/assets/prose.css";
            }
            body {
                (content)
                script type="module" src="/assets/app.js" {}
                script type="module" src="/assets/islands.js" {}
            }
        }
    }
}
