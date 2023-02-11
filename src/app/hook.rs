use yew::prelude::*;
use yew_router::prelude::*;
use yew_hooks::prelude::*;
use serde::{Serialize, Deserialize};

use crate::app::routes::Route;

#[derive(Debug, Serialize)]
struct CheckRequest {
    master_password: String
}

#[derive(Debug, Deserialize)]
struct CheckResponse {
    success: bool
}

#[derive(PartialEq, Default, Clone)]
pub struct UserInfo {
    pub password: String
}

pub struct UseUserContextHandle {
    pub inner: UseStateHandle<UserInfo>,
    pub navigator: Navigator
}

impl UseUserContextHandle {
    pub fn login(&self, user_info: UserInfo) -> Result<(), String> {
        {
            let info = user_info.clone();
            use_async(async move {
                let check = CheckRequest {
                    master_password: info.password
                };

                let client = reqwest::Client::new();
                let res = client.post("http://127.0.0.1:8080/check")
                    .header("Content-Type", "application/json")
                    .json(&check)
                    .send()
                    .await;

                if let Ok(data) = res {
                    let data: CheckResponse = data.json().await.unwrap();

                    if data.success {
                        Ok(())
                    }
                    else {
                        Err("Error".to_string())
                    }
                }
                else {
                    Err("Error".to_string())
                }
            });
        }

        self.inner.set(user_info);
        self.navigator.push(&Route::Home);
        Ok(())
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

    UseUserContextHandle {inner, navigator}
}
