mod components {
    pub mod home;
}

mod utils;

use components::home::Home;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Home />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
