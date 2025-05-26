use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::navbar::Navbar;
use crate::pages::home::Home;

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    //TODO
    //Implement pages
    #[at("/about")]
    About,
    #[at("/")]
    Home,
    #[at("/research")]
    Research,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
}

pub fn switch(route: AppRoute) -> Html {
    match route     {
        AppRoute::Home => html! {<Home />},
        AppRoute::About => html! {"TODO"},
        AppRoute::Research => html! {"TODO"},
        AppRoute::Projects => html! {"TODO"},
        AppRoute::Contact => html! {"TODO"},
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <HashRouter>
            <Navbar />
            <Switch<AppRoute> render={switch} />
        </HashRouter>
    }
}
