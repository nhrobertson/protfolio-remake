use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::card::Card;
use crate::app::AppRoute;

#[function_component]
pub fn Games() -> Html {
    html! {
        <div class="games-container">
            <Card title="Bale's Tale">
                <img src="" style="width:100px; height:auto;" />
                <p>{"Bale's Tale is a small 2D platformer created in a Month for a Computer Game Development Course. It was worked on by the Roarring Star Studio Team. I orginated the idea and implemented a great deal of the features. With the small nature of the game I was able to get a demo up and running in WebGL. Take a look!"}</p>
                <Link<AppRoute> to={AppRoute::BalesTale} classes="nav-link">{ "Bale's Tale Demo"}</Link<AppRoute>>
            </Card>
            <Card title="Crawfish Creek">
                <img src="" style="width:100px; height:auto;" />
                <p>{"Crawfish Creek is a 3D infinite scroller also developed for my Computer Game Development Course. It was also created by the Roarring Star Studio Team. Feel free to take a look!"}</p>
                <Link<AppRoute> to={AppRoute::BalesTale} classes="nav-link">{ "Crawfish Creek Demo"}</Link<AppRoute>>
            </Card>
            <Card title="Hello World!">
                <img src="" style="width:100px; height:auto;" />
                <p>{"Working with a new team, look out for our next title: Hello World! We have full plans to eventually release on steam."}</p>
            </Card>
        </div>
    }
}
