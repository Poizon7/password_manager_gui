use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <h6>{"Spectrum Password Manager"}</h6>
        </header>
    }
}
