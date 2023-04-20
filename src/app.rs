use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod hook;
mod routes;

use crate::app::components::{header::Header, user_context_provider::UserContextProvider};
use crate::app::routes::{switch, Route};

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
