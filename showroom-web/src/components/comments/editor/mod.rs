use boutique::html;
use bq_components::component;
use maud::Markup;

#[component(setup = "comment-editor")]
pub fn comment_editor(#[positional] body: &str, #[positional] content: &str, max: u64) -> Markup {
    html! {
        div.comment-editor bq-setup {
            textarea
                bq-ref="fallback"
                name="body"
                aria-label="Comment"
                placeholder="Write a comment…"
                rows="3"
                maxlength=(max)
                required
            {
                (body)
            }
            input bq-ref="content" type="hidden" name="content" value=(content) disabled;
            div.comment-editor-surface bq-ref="surface" hidden {}
        }
    }
}
