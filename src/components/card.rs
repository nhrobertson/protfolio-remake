use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    html ! {
        <div class={classes!("card", &props.class)}>
            if !props.title.is_empty() {
                <div class="card-header">
                    <h3 class="card-h3">{ &props.title }</h3>
                </div>
            }
            <div class="card-body">
                { for props.children.iter() }
            </div>
        </div>
    }
}
