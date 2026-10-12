use boutique::html;
use bq_components::{Attrs, component, setup};
use maud::Markup;

setup!(Menu);

#[component]
pub fn menu(
    #[builder(start_fn)] label: &str,
    button: Markup,
    anchor: Option<&str>,
    #[builder(default)] attrs: Attrs,
    children: Markup,
) -> Markup {
    html! {
        div.menu bq-setup=(Menu) data-anchor=[anchor] {
            button
                type="button"
                aria-haspopup="menu"
                aria-expanded="false"
                aria-label=(label)
                title=(label)
                bq-ref="button"
                bq-on:mousedown|prevent
                bq-on:click="toggle"
                bq-on:keydown|down|up|prevent|stop="open-from-key"
                (..attrs)
            {
                (button)
            }
            div.menu-panel
                role="menu"
                aria-label=(label)
                hidden
                bq-ref="panel"
                bq-on:mousedown|prevent
                bq-on:click="choose"
                bq-on:keydown="navigate"
                bq-on:pointerdown|document="outside"
                bq-on:resize|window="position"
                bq-on:scroll|window|passive="position"
            {
                (children)
            }
        }
    }
}
