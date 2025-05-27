use std::time::Duration;
use yew::prelude::*;
use yew_hooks::use_interval;

#[function_component]
pub fn Carousel() -> Html {
    let slides = vec![
        html! { 
            <div class="slide slide-a" >
                <h3>{"Computer Engineer"}</h3>
                <p>{"Obtained a Bachelors of Science in Computer Engineering in 2024"}</p>
            </div> 
        },
        html! { 
            <div class="slide slide-b" >
                <h3>{"Researcher"}</h3>
                <p>{"Has been a part of various research projects resulting in published papers"}</p>
            </div> 
        },
        html! { 
            <div class="slide slide-c" >
                <h3>{"Game Developer"}</h3>
                <p>{"Works on games in free time. Looking to publish soon!"}</p>
            </div> },
    ];
    let len = slides.len();
    let slide_idx = use_state(|| 0_usize);

    let interval_ms = 5000;
    {
        // clone handle for the closure
        let idx = slide_idx.clone();
        // bump every 3 seconds
        use_interval(
            move || {
                idx.set(((*idx + 1) % len) as usize);
            },
            interval_ms,
        );
    }

    html! {
        <div class="carousel-container">
            { for slides.into_iter().enumerate().map(|(i, slide)| {
                if i == *slide_idx { slide } else { html!{} }
            }) }
        </div>
    }
}
