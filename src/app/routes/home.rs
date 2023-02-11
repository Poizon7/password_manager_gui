use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::app::routes::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = {
        Callback::from(move |_| {
            navigator.push(&Route::Login);
        })
    };
    html!{
        <main>
            <h2>{"Home"}</h2>
            <button onclick={onclick}>{"Logout"}</button>
        </main>
    }
}
