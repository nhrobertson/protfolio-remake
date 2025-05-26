use std::time::Duration;
use yew::prelude::*;
use yew_hooks::use_interval;

#[function_component]
pub fn Carousel() -> Html {
    let slides = vec![
        html! { <div class="slide">{"TODO A"}</div> },
        html! { <div class="slide">{"TODO B"}</div> },
        html! { <div class="slide">{"TODO C"}</div> },
    ];
    let len = slides.len();
    let slide_idx = use_state(|| 0_usize);

    let interval_ms = 3000;
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
