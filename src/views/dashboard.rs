use html_to_string_macro::html;
use super::layouts::base;

pub fn index(is_authenticated: bool, base_url: &str) -> String {
    base(html!(
        <div>
            <h1>"your dashboard"</h1>
            {if is_authenticated {
                html!(<form method="POST" action="/logout"><button type="submit">"Sign out"</button></form>)
            } else {
                html!(<a href={format!("{}/login", base_url)}>"Sign in"</a>)
            }}
        </div>
    ))
}
