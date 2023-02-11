use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html!{
        <header>
            <h1>{"Spectrum Password Manager"}</h1>
        </header>
    }
}
