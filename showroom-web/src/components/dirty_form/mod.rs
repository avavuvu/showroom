use boutique::html;
use bq_components::{Attrs, component};
use maud::Markup;

#[component]
pub fn dirty_form(
    #[positional] action: &str,
    attrs: Attrs,
    children: Markup,
) -> Markup {
    html! {
        form
            method="POST"
            action=(action)
            bq-setup
            bq-on:input="dirty-form.update"
            bq-on:change="dirty-form.update"
            bq-on:reset="dirty-form.reset"
            bq-on:submit="dirty-form.submit"
            bq-on:beforeunload|window="dirty-form.leave"
            bq-on:pageshow|window="dirty-form.restore"
            (..attrs)
        {
            (children)
        }
    }
}
