use yew::prelude::*;

use crate::components::tabitem::TabItem;
use crate::components::card::Card;

#[function_component]
pub fn Research() -> Html {
    html! {
        <div class="container">
            <h2>{"Research"}</h2>
            <p>{"TODO"}</p>
            <div class="tabitem-container">
                <TabItem title="TODO 1">
                    <div class="tabitem-body-container">
                        <h3>{"Title"}</h3>
                        <p>{"TODO"}</p>
                        <Card title="TODO 1">
                            <h3>{"TODO"}</h3>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="TODO 2">
                    <div class="tabitem-body-container">
                        <h3>{"Title"}</h3>
                        <p>{"TODO"}</p>
                        <Card title="TODO 2">
                            <h3>{"TODO"}</h3>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="TODO 3">
                    <div class="tabitem-body-container">
                        <h3>{"Title"}</h3>
                        <p>{"TODO"}</p>
                        <Card title="TODO 3">
                            <h3>{"TODO"}</h3>
                        </Card>
                    </div>
                </TabItem>
            </div>
        </div>
    }
}
