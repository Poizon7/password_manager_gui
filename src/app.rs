use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
mod components;
mod hook;

use crate::app::routes::{switch, Route};
use crate::app::components::{Header, UserContextProvider};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <UserContextProvider>
                <Header />
                <Switch<Route> render={switch} />
            </UserContextProvider>
        </BrowserRouter>
    }
}
