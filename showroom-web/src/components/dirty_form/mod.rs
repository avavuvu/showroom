use boutique::html;
use bq_components::{Attrs, component, setup};
use maud::Markup;

setup!(DirtyForm);

#[component]
pub fn dirty_form(
    #[builder(start_fn)] action: &str,
    #[builder(default)] attrs: Attrs,
    #[builder(default = html! {})] children: Markup,
) -> Markup {
    html! {
        form
            method="POST"
            action=(action)
            bq-setup=(DirtyForm)
            bq-on:input="update"
            bq-on:change="update"
            bq-on:reset="reset"
            bq-on:submit="submit"
            bq-on:beforeunload|window="leave"
            bq-on:pageshow|window="restore"
            (..attrs)
        {
            (children)
        }
    }
}
