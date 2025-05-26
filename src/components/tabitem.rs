use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabItemProps {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component]
pub fn TabItem(props: &TabItemProps) -> Html {
    let show_item = use_state(|| false);
    let onclick = {
        let show_item = show_item.clone();
        Callback::from(move |_| show_item.set(!*show_item))
    };

    html! {
        <div class={classes!("tabitem", &props.class)}>
            <button {onclick} class="tabitem-button">{ &props.title }</button>
            if *show_item {
                <div class="tabitem-body">
                    { for props.children.iter() }
                </div>
            }
        </div>
    }
}
