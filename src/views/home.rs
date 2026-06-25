use html_to_string_macro::html;
use super::layouts::base;

pub fn index() -> String {
    base(html!(
        <div>
            <div>
                <h1>"Showroom"</h1>
                <p>"Newsletters for a new age"</p>
                <div>
                    <a href="/login">"Sign in"</a>
                    <a href="/signup">"Sign up"</a>
                </div>
            </div>
        </div>
    ))
}
