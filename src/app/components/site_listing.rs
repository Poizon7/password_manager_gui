use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub site: String,
    pub onclick: Callback<String>
}


#[function_component(SiteListing)]
pub fn site_listing(props: &Props) -> Html {
    let onclick = {
        let props = props.clone();
        Callback::from(move |_| {
            props.onclick.emit(props.site.clone());
        })
    };
    html! {
        <section class="site">
            <h2>{&*props.site.clone()}</h2>
            <button class="filled" onclick={onclick}>{"View"}</button>
        </section>
    }
}
