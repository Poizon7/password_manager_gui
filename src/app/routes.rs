use yew::prelude::*;
use yew_router::*;

mod home;
mod login;

use home::Home;
use login::Login;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{<Home />},
        Route::Login => html!{<Login />},
        Route::NotFound => html!{<h1>{"Not found"}</h1>}
    }
}
