use html_to_string_macro::html;

pub fn base(content: String) -> String {
    format!(
        "<!DOCTYPE html>{}",
        html!(
            <html lang="en">
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>"Showroom"</title>
                    <link rel="stylesheet" href="/assets/styles.css" />
                    <link rel="stylesheet" href="/assets/prose.css" />
                </head>
                <body>
                    {content}
                    <script type="module" src="/assets/app.js"></script>
                    <script type="module" src="/assets/islands.js"></script>
                </body>
            </html>
        )
    )
}
