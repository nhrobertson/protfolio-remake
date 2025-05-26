use yew::prelude::*;

use crate::components::carousel::{Carousel};
use crate::components::card::{Card};

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="container">
            <Carousel />
            { upper_home() }
            { lower_home() }
        </div>
    }
}

fn upper_home() -> Html {
    html! {
        <div class="upper-home-container">
            <img src="" alt="TODO" />
            <div class="home-about-text">
                <h2 class="home-h2">{"Noah Robertson"}</h2>
                <p class="home-p">{"TODO"}</p>
            </div>
            <Card title="Links" class="links-card">
                {"TODO"}
            </Card>
        </div>
    }
}

fn lower_home() -> Html {
    html! {
        <div class="lower-home-container">
            <Card title="Card 1" class="home-card">
                <img src="" alt="Card Img TODO" />
                <p class="card-p">{"TODO"}</p>
            </Card>
            <Card title="Card 2" class="home-card">
                <img src="" alt="Card Img TODO" />
                <p class="card-p">{"TODO"}</p>
            </Card>
            <Card title="Card 3" class="home-card">
                <img src="" alt="Card Img TODO" />
                <p class="card-p">{"TODO"}</p>
            </Card>
        </div>
    }
}
