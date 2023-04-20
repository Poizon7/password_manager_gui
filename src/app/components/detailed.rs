use yew::prelude::*;
use yew_hooks::use_async;

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;

use serde::{Serialize, Deserialize};
use reqwest::StatusCode;

use crate::app::hook::use_user_context;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize)]
struct GetRequest {
    site: String,
    master_password: String,
}

#[derive(Default, Clone, Deserialize)]
struct GetResponse {
    username: String,
    password: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub site: String,
}

#[derive(Serialize)]
struct CopyArgs {
    name: String
}

#[function_component(Detailed)]
pub fn detailed(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let site_info = use_state(|| GetResponse::default());

    let get_password = {
        let props = (*props).clone();
        let site_info = site_info.clone();

        use_async(async move {
            let check = GetRequest {
                site: props.site.to_string(),
                master_password: user_ctx.inner.password.clone(),
            };

            let client = reqwest::Client::new();
            let res = client
                .post("http://127.0.0.1:8080/get")
                .header("Content-Type", "application/json")
                .json(&check)
                .send()
                .await
                .unwrap();

            if res.status() == StatusCode::OK {
                let info = res.json().await.unwrap();

                site_info.set(info);
                Ok(())
            } else {
                Err(res.status())
            }
        })
    };

    use_effect_with_deps(move |_| {get_password.run()}, props.site.clone());


    let onclick_copy = {
        let site_info = site_info.clone();

        Callback::from(move |_| {
            let site_info = site_info.clone();
            spawn_local(async move {
                invoke("copy", to_value(&CopyArgs{name: site_info.password.clone()}).unwrap()).await;
            })
        })
    };

    html! {
        <div id="detailed">
            <div class="information_section">
                <section>
                    <h3>{"Name"}</h3>
                    <h1>{props.site.clone()}</h1>
                </section>
                <hr/>
                <section>
                    <h3>{"Username"}</h3>
                    <h2>{site_info.username.clone()}</h2>
                </section>
                <hr/>
                <section>
                    <h3>{"Password"}</h3>
                    <div class="field">
                        <h2>{site_info.password.clone()}</h2>
                        <button class="copy" onclick={onclick_copy}><img src="../../../public/content_copy.svg" /></button>
                    </div>
                </section>
            </div>
            <div class="information_section">
                <section>
                    <h3>{"URL"}</h3>
                    <h2>{"Comming"}</h2>
                </section>
            </div>
            <div>
                <section>
                    <button class="filled">{"Update"}</button>
                    <button class="filled delete">{"Delete"}</button>
                </section>
            </div>
        </div>
    }
}
