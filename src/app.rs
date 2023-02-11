use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
mod components;

use crate::app::routes::{switch, Route};
use crate::app::components::Header;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
