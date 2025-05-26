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
            <img class="home-img" src="assets/home-pic-1.png" alt="TODO" />
            <div class="home-about-text">
                <h2 class="home-h2">{"Noah Robertson"}</h2>
                <p class="home-p">{"Hello! My name is Noah Robertson. I am a computer engineer who likes to tinker with electronics and write software! Welcome to my portfolio. Here you can find information about me, research I have been involved with, projects I have be a part of/have done, and ways to contact me. Make sure to check me out on LinkedIn and GitHub!"}</p>
            </div>
            <Card title="Links" class="links-card">
                <div class="links-card-container">
                    <a href="https://www.linkedin.com/in/noah-robertson-330331211/"><img class="card-img" src="https://upload.wikimedia.org/wikipedia/commons/thumb/8/81/LinkedIn_icon.svg/1024px-LinkedIn_icon.svg.png" alt="LinkedIn" /></a>
                    <a href="https://github.com/nhrobertson"><img class="card-img" src="https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png" alt="GitHub" /></a>
                </div>
            </Card>
        </div>
    }
}

fn lower_home() -> Html {
    html! {
        <div class="lower-home-container">
            <Card title="Card 1" class="home-card">
                <img class="card-img" src="" alt="Card Img TODO" />
                <p class="card-p">{"TODO"}</p>
            </Card>
            <Card title="Card 2" class="home-card">
                <img class="card-img" src="" alt="Card Img TODO" />
                <p class="card-p">{"TODO"}</p>
            </Card>
            <Card title="Card 3" class="home-card">
                <img class="card-img" src="" alt="Card Img TODO" />
                <p class="card-p">{"TODO"}</p>
            </Card>
        </div>
    }
}
