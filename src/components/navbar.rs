use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav>
            <div class="container">
                <img src="assets/Logo.png" alt="logo" class="nav-logo"/>
                <ul class="nav-list">
                    <li class="nav-item">
                        <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                            { "Home" }
                        </Link<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <Link<AppRoute> to={AppRoute::About} classes="nav-link">
                            { "About" }
                        </Link<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <Link<AppRoute> to={AppRoute::Research} classes="nav-link">
                            { "Research" }
                        </Link<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <Link<AppRoute> to={AppRoute::Projects} classes="nav-link">
                            { "Projects" }
                        </Link<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <Link<AppRoute> to={AppRoute::Games} classes="nav-link">
                            { "Games" }
                        </Link<AppRoute>>
                    </li>
                    <li class="nav-item">
                        <Link<AppRoute> to={AppRoute::Contact} classes="nav-link">
                            { "Contact" }
                        </Link<AppRoute>>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
