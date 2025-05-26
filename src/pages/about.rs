use yew::prelude::*;

#[function_component]
pub fn About() -> Html {
    html !{
        <div class="container">
            <div class="about-item-left">
                <img src="" alt="Me" />
                <div class="about-text">
                    <h3>{"About"}</h3>
                    <p>{"TODO"}</p>
                </div>
            </div>
            <div class="about-item-right">
                <div class="about-text">
                    <h3>{"Academics"}</h3>
                    <p>{"TODO"}</p>
                </div>
                <img src="" alt="Me" />
            </div>
            <div class="about-item-left">
                <img src="" alt="Me" />
                <div class="about-text">
                    <h3>{"Current"}</h3>
                    <p>{"TODO"}</p>
                </div>
            </div>
        </div>
    }
}
