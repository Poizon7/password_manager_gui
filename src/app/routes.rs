use yew::prelude::*;
use yew_router::*;

mod add;
mod home;
mod login;

use add::Add;
use home::Home;
use login::Login;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/add")]
    Add,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Login => html! {<Login />},
        Route::Add => html! {<Add />},
        Route::NotFound => html! {<h1>{"Not found"}</h1>},
    }
}
