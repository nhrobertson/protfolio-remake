pub mod app;
pub mod components;
pub mod pages;

use crate::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
