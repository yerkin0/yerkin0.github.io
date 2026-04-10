mod models;
mod components;
mod app; 

use app::App; 

fn main() {
    yew::Renderer::<App>::new().render();
}