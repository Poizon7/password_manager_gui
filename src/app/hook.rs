use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use spectrum::cryptography::RSA;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::routes::Route;

#[derive(Debug, Serialize)]
struct CheckRequest {
    master_password: String,
}

#[derive(Debug, Deserialize)]
struct PublicKeyResponse {
    n: u128,
    e: u128
}

#[derive(PartialEq, Default, Clone)]
pub struct UserInfo {
    pub password: String,
    pub current_site: String,
}

#[derive(Clone)]
pub struct UseUserContextHandle {
    pub inner: UseStateHandle<UserInfo>,
    pub navigator: Navigator,
}

impl UseUserContextHandle {
    pub async fn login(&self, user_info: UserInfo) -> Result<(), StatusCode> {
        let info = user_info.clone();

        let client = reqwest::Client::new();

        let res = client
            .get("http://127.0.0.1:8080/getPublicKey")
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap();

        let public_key: PublicKeyResponse = res.json().await.unwrap();

        let rsa = RSA::from_num(public_key.n, public_key.e, 0);

        let check = CheckRequest {
            master_password: info.password,
        };

        let res = client
            .post("http://127.0.0.1:8080/check")
            .header("Content-Type", "application/json")
            .json(&check)
            .send()
            .await
            .unwrap();

        if res.status() == StatusCode::OK {
            self.inner.set(user_info);
            self.navigator.push(&Route::Home);
            Ok(())
        } else {
            Err(res.status())
        }
    }

    pub fn logout(&self) {
        self.inner.set(UserInfo::default());
        self.navigator.push(&Route::Login);
    }
}

#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let navigator = use_navigator().unwrap();

    UseUserContextHandle { inner, navigator }
}
