use yew::prelude::*;

use crate::app::hook::UserInfo;

#[function_component(Header)]
pub fn header() -> Html {
    html!{
        <header>
            <h1>{"Spectrum Password Manager"}</h1>
        </header>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(|| UserInfo::default());

    html!{
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            {for props.children.iter()}
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
