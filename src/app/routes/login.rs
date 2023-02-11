use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::app::hook::{use_user_context, UserInfo};


#[function_component(Login)]
pub fn login() -> Html {
    let user_ctx = use_user_context();
    let user_info = use_state(|| UserInfo::default());

    let onsubmit = {
        let user_info = user_info.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let info = (*user_info).clone();
            user_ctx.login(info).unwrap();
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

    html!{
        <main>
            <form {onsubmit}>
                <fieldset>
                    <legend>{"Login"}</legend>
                    <lable for="pwd">
                        <span>{"Password: "}</span>
                        <strong><span aria-label="required">{"*"}</span></strong>
                    </lable>
                    <input type="password" id="pwd" name="password" value={user_info.password.clone()} oninput={oninput} />
                    <button>{"Login"}</button>
                </fieldset>
            </form>
        </main>
    }
}
