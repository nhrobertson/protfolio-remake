use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::navbar::Navbar;
use crate::pages::home::Home;
use crate::pages::about::About;
use crate::pages::research::Research;
use crate::pages::projects::Projects;
use crate::pages::games::Games;
use crate::pages::balestale::BalesTale;
use crate::pages::contact::Contact;

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
    #[at("/games")]
    Games,
    #[at("/balestale")]
    BalesTale,
}

pub fn switch(route: AppRoute) -> Html {
    match route     {
        AppRoute::Home => html! {<Home />},
        AppRoute::About => html! {<About />},
        AppRoute::Research => html! {<Research />},
        AppRoute::Projects => html! {<Projects />},
        AppRoute::Games => html! {<Games />},
        AppRoute::Contact => html! {<Contact />},
        AppRoute::BalesTale => html! {<BalesTale />},
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
