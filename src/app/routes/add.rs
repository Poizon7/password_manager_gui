use reqwest::StatusCode;
use serde::Serialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

use crate::app::hook::use_user_context;
use crate::app::routes::Route;

#[derive(Default, Clone)]
struct AddInfo {
    site: String,
    username: String,
    password: String,
    master_password: String,
    generate: bool,
}

#[derive(Serialize)]
struct GenRequest {
    site: String,
    username: String,
    master_password: String,
}

#[derive(Serialize)]
struct SetRequest {
    site: String,
    username: String,
    password: String,
    master_password: String,
}

#[function_component(Add)]
pub fn add() -> Html {
    let user_ctx = use_user_context();
    let add_info = use_state(|| AddInfo::default());

    let gen = {
        let user_ctx = user_ctx.clone();
        let add_info = add_info.clone();
        use_async(async move {
            let gen = GenRequest {
                site: add_info.site.clone(),
                username: add_info.username.clone(),
                master_password: add_info.master_password.clone(),
            };

            let client = reqwest::Client::new();
            let res = client
                .post("http://127.0.0.1:8080/gen")
                .header("Content-Type", "application/json")
                .json(&gen)
                .send()
                .await
                .unwrap();

            if res.status() == StatusCode::OK {
                user_ctx.navigator.push(&Route::Home);
                Ok(())
            } else {
                Err(())
            }
        })
    };

    let set = {
        let user_ctx = user_ctx.clone();
        let add_info = add_info.clone();
        use_async(async move {
            let set = SetRequest {
                site: add_info.site.clone(),
                username: add_info.username.clone(),
                password: add_info.password.clone(),
                master_password: add_info.master_password.clone(),
            };

            let client = reqwest::Client::new();
            let res = client
                .post("http://127.0.0.1:8080/set")
                .header("Content-Type", "application/json")
                .json(&set)
                .send()
                .await
                .unwrap();

            if res.status() == StatusCode::OK {
                user_ctx.navigator.push(&Route::Home);
                Ok(())
            } else {
                Err(())
            }
        })
    };

    let onsubmit = {
        let add_info = add_info.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if add_info.generate {
                gen.run();
            } else {
                set.run();
            }
        })
    };

    let oninput_site = {
        let add_info = add_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*add_info).clone();
            info.site = input.value();
            add_info.set(info);
        })
    };

    let oninput_username = {
        let add_info = add_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*add_info).clone();
            info.username = input.value();
            add_info.set(info);
        })
    };

    let oninput_password = {
        let add_info = add_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*add_info).clone();
            info.password = input.value();
            add_info.set(info);
        })
    };

    let oninput_master_password = {
        let add_info = add_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*add_info).clone();
            info.master_password = input.value();
            add_info.set(info);
        })
    };

    let oninput_generate = {
        let add_info = add_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*add_info).clone();
            if input.value() == "gen" {
                info.generate = true;
            } else {
                info.generate = false;
            }
            add_info.set(info);
        })
    };

    let onclick = {
        Callback::from(move |_| {
            user_ctx.navigator.push(&Route::Home);
        })
    };

    html! {
        <main>
            <button class="back_button" onclick={onclick}>{"< Back"}</button>
            <form {onsubmit}>
                <h1>{"Add New Password"}</h1>
                <fieldset>
                    <legend>{"Info"}</legend>
                    <lable class="outlined" for="site">
                        <input type="text" id="site" name="site" placeholder=" " value={add_info.site.clone()} oninput={oninput_site} />
                        <span>{"Site"}</span>
                    </lable>
                    <lable class="outlined" for="username">
                        <input type="text" id="username" name="username" placeholder=" " value={add_info.username.clone()} oninput={oninput_username} />
                        <span>{"Username"}</span>
                    </lable>
                </fieldset>
                <fieldset>
                    <legend>{"Password"}</legend>
                    if !add_info.generate {
                            <lable class="outlined" for="password">
                                <input type="password" id="password" name="password" placeholder=" " value={add_info.password.clone()} oninput={oninput_password} />
                                <span>{"Password"}</span>
                            </lable>
                    }
                    <div class="radio_button">
                        <lable for="set">
                            <input type="radio" id="set" name="add_type" checked={!add_info.generate} value={"set"} oninput={oninput_generate.clone()} />
                            {"Set"}
                        </lable>
                        <lable for="gen">
                            <input type="radio" id="gen" name="add_type" checked={add_info.generate} value={"gen"} oninput={oninput_generate} />
                            {"Gen"}
                        </lable>
                    </div>
                </fieldset>
                <fieldset>
                    <legend>{"Master Password"}</legend>
                    <lable class="outlined" for="master_password">
                        <input type="password" id="master_password" name="master_password" placeholder=" " value={add_info.master_password.clone()} oninput={oninput_master_password} />
                        <span>{"Master Password"}</span>
                    </lable>
                </fieldset>
                <button>{"Add"}</button>
            </form>
        </main>
    }
}
