use html_to_string_macro::html;
use super::layouts::base;

pub fn login(error: Option<&str>) -> String {
    base(html!(
        <div>
            <div>
                <h1>"Sign in"</h1>
                {error.map(|err| html!(<p>{err}</p>)).unwrap_or_default()}
                <form method="POST" action="/login" novalidate data-controller="login-form">
                    <div>
                        <label for="email">"Email"</label>
                        <input
                            id="email"
                            name="email"
                            type="email"
                            autocomplete="email"
                            data-login-form-target="email"
                            data-action="blur->login-form#validateEmail"
                        />
                        <p hidden data-login-form-target="emailError"></p>
                    </div>
                    <div>
                        <label for="password">"Password"</label>
                        <div>
                            <input
                                id="password"
                                name="password"
                                type="password"
                                autocomplete="current-password"
                                data-login-form-target="password"
                            />
                            <button
                                type="button"
                                tabindex="-1"
                                data-login-form-target="passwordToggle"
                                data-action="click->login-form#togglePassword"
                            >"Show"</button>
                        </div>
                    </div>
                    <button type="submit">"Sign in"</button>
                </form>
                <p>"No account? "<a href="/signup">"Sign up"</a></p>
            </div>
        </div>
    ))
}

pub fn signup(error: Option<&str>) -> String {
    base(html!(
        <div>
            <div>
                <h1>"Create account"</h1>
                {error.map(|err| html!(<p>{err}</p>)).unwrap_or_default()}
                <form method="POST" action="/signup" novalidate data-controller="login-form">
                    <div>
                        <label for="email">"Email"</label>
                        <input
                            id="email"
                            name="email"
                            type="email"
                            autocomplete="email"
                            data-login-form-target="email"
                            data-action="blur->signup-form#validateEmail"
                        />
                        <p hidden data-login-form-target="emailError"></p>
                    </div>
                    <div>
                        <label for="handle">"Handle"</label>
                        <input
                            id="handle"
                            name="handle"
                            type="text"
                            autocomplete="handle"
                            data-login-form-target="handle"
                            data-action="blur->signup-form#validateHandle"
                        />
                        <p hidden data-login-form-target="handleError"></p>
                    </div>
                    <div>
                        <label for="password">"Password"</label>
                        <div>
                            <input
                                id="password"
                                name="password"
                                type="password"
                                autocomplete="new-password"
                                data-login-form-target="password"
                            />
                            <button
                                type="button"
                                tabindex="-1"
                                data-signup-form-target="passwordToggle"
                                data-action="click->signup-form#togglePassword"
                            >"Show"</button>
                        </div>
                    </div>
                    <button type="submit">"Create account"</button>
                </form>
                <p>"Already have an account? "<a href="/login">"Sign in"</a></p>
            </div>
        </div>
    ))
}
