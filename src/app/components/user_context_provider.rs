use crate::app::hook::UserInfo;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ContextProviderProps {
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &ContextProviderProps) -> Html {
    let user_ctx = use_state(|| UserInfo::default());

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            {for props.children.iter()}
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
