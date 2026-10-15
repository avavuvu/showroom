use boutique::html;
use bq_components::{Attrs, component, setup};
use maud::Markup;

setup!(Menu);

#[component]
pub fn menu(
    #[builder(start_fn)] label: &str,
    id: &str,
    button: Markup,
    #[builder(default)] attrs: Attrs,
    children: Markup,
) -> Markup {
    let anchor = format!("--{id}");
    html! {
        div.menu bq-setup=(Menu) {
            button
                type="button"
                aria-haspopup="menu"
                aria-label=(label)
                title=(label)
                popovertarget=(id)
                style={ "anchor-name: " (anchor) }
                bq-ref="button"
                bq-on:mousedown|prevent
                bq-on:click="remember"
                bq-on:keydown|down|up|prevent|stop="open-from-key"
                (..attrs)
            {
                (button)
            }
            div.menu-panel
                #(id)
                popover
                role="menu"
                aria-label=(label)
                style={ "position-anchor: " (anchor) }
                bq-ref="panel"
                bq-on:mousedown|prevent
                bq-on:click="choose"
                bq-on:keydown="navigate"
                bq-on:toggle="opened"
            {
                (children)
            }
        }
    }
}
