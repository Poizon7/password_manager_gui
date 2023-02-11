use yew::prelude::*;
use crate::app::hook::use_user_context;

#[function_component(Home)]
pub fn home() -> Html {
    let user_ctx = use_user_context();

    let onclick = {
        Callback::from(move |_| {
            user_ctx.logout();
        })
    };
    html!{
        <main>
            <h2>{"Home"}</h2>
            <button onclick={onclick}>{"Logout"}</button>
        </main>
    }
}
