use yew::prelude::*;

#[function_component]
pub fn BalesTale() -> Html {
    html! {
        <div class="container">
            <iframe src="assets/balestale/index.html" style="width:100%; height:100vh; border:none;" title="Bale's Tale" />
        </div>
    }
}
