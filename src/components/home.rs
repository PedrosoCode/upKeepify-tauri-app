use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use crate::utils::get_pings;

#[function_component(Home)]
pub fn home() -> Html {
    let pings = use_state(|| vec![]);
    {
        let pings = pings.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                match get_pings().await {
                    Ok(data) => pings.set(data),
                    Err(error) => console::log_1(&error),
                };
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <h1>{ "Pings from Database" }</h1>
            <ul>
                { for (*pings).iter().map(|ping| html! { <li key={ping.id}>{ format!("{}: {}", ping.id, ping.value) }</li> }) }
            </ul>
        </div>
    }
}
