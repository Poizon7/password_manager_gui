use crate::app::hook::{use_user_context, UserInfo};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

#[function_component(Login)]
pub fn login() -> Html {
    let user_ctx = use_user_context();
    let user_info = use_state(|| UserInfo::default());

    let login = {
        let user_info = user_info.clone();
        use_async(async move {
            let info = (*user_info).clone();
            user_ctx.login(info).await
        })
    };

    let onsubmit = {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            login.run();
        })
    };

    let oninput = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.password = input.value();
            user_info.set(info);
        })
    };

    html! {
        <main>
            <form {onsubmit}>
                <fieldset>
                    <legend><h2>{"Login"}</h2></legend>
                    <lable class="outlined" for="password">
                        <input type="password" id="password" name="password" value={user_info.password.clone()} oninput={oninput} placeholder=" " />
                        <span>{"Password"}</span>
                    </lable>
                </fieldset>
                <button class="filled">{"Login"}</button>
            </form>
        </main>
    }
}
