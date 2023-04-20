use crate::app::hook::use_user_context;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::use_async;

use crate::app::components::{site_listing::SiteListing, detailed::Detailed};
use crate::app::routes::Route;

#[derive(Serialize)]
struct ShowRequest {
    master_password: String,
}

#[derive(Default, Deserialize, Debug)]
struct ShowResponse {
    sites: Vec<String>,
}

#[function_component(Home)]
pub fn home() -> Html {
    let user_ctx = use_user_context();
    let sites = use_state(|| ShowResponse::default());
    let current_site = use_state(|| String::default());

    let get = {
        let user_ctx = user_ctx.clone();
        let sites = sites.clone();

        use_async(async move {
            let show = ShowRequest {
                master_password: user_ctx.inner.password.clone(),
            };

            let client = reqwest::Client::new();
            let res = client
                .post("http://127.0.0.1:8080/show")
                .header("Content-Type", "application/json")
                .json(&show)
                .send()
                .await
                .unwrap();

            if res.status() == StatusCode::OK {
                let info = res.json().await.unwrap();

                sites.set(info);
                Ok(())
            } else {
                user_ctx.logout();
                Err(res.status())
            }
        })
    };

    use_effect_with_deps(move |_| get.run(), ());

    let onclick_add = {
        let user_ctx = user_ctx.clone();
        Callback::from(move |_| {
            user_ctx.navigator.push(&Route::Add);
        })
    };

    let onclick_logout = {
        Callback::from(move |_| {
            user_ctx.logout();
        })
    };

    let onclick_view = {
        let current_site = current_site.clone();
        Callback::from(move |site: String| {
            current_site.set(site);
        })
    };

    html! {
        <main id="home">
            <aside>
                <ul id="passwords">{
                    sites.sites.iter().map(|site| {
                        html!{<SiteListing site={site.clone()} onclick={onclick_view.clone()} />}
                    }).collect::<Html>()
                }</ul>
                <button class="filled" onclick={onclick_add}>{"Add"}</button>
            </aside>
            <div id="main">
                if !(*current_site).is_empty() {
                    <Detailed site={(*current_site).clone()}/>
                }
                else {
                    <div></div>
                }
                <img src="../../../public/raven.png" />
            </div>
            <div id="controls">
                <button class="filled" onclick={onclick_logout}>{"Logout"}</button>
            </div>
        </main>
    }
}
