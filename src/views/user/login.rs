use dioxus::prelude::*;
use dioxus_i18n::tid;

use super::components::{EmailInput, PasswordInput};
use crate::{
    app::AppGlobalState,
    components::Alert,
    shared::user::{Credentials, LoggedUser},
};

#[component]
pub fn Login() -> Element {
    let mut alert = use_context::<AppGlobalState>();
    let mut logged = use_context::<Signal<Option<LoggedUser>>>();
    let nav = use_navigator();

    let form_submit = move |evt: Event<FormData>| {
        let redirect: String = alert.redirect.read().clone();
        evt.prevent_default();
        let values = evt.values();
        let payload = Credentials {
            email: values
                .get("email")
                .and_then(|v| v.first())
                .cloned()
                .unwrap_or_default(),
            password: values
                .get("password")
                .and_then(|v| v.first())
                .cloned()
                .unwrap_or_default(),
        };

        async move {
            tracing::debug!("sending to server");
            let response = crate::shared::user::login_user(payload.clone()).await;
            match response {
                Ok(Some(user)) => {
                    logged.set(Some(user.clone()));
                    alert.alert.set(Some((
                        Alert::Info,
                        tid!("login.suc", username: user.user.email),
                    )));
                    tracing::debug!("it should redirect to: {}", &redirect);
                    nav.push(redirect);
                }
                Err(e) => {
                    alert.alert.set(Some((Alert::Error, e.to_string())));
                }
                Ok(None) => {
                    tracing::warn!("login response is empty");
                }
            }
        }
    };

    let login_label = tid!("login");
    rsx! {
        div {
            class: "flex justify-center items-center min-h-screen",
            form {
                // a fix for bug [prevent_default()](https://github.com/DioxusLabs/dioxus/issues/4303)
                action: "#",
                method: "dialog",
                onsubmit: form_submit,
                input {
                    r#type: "text",
                    name: "next",
                    required: "false",
                }
                fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4",
                    legend { class: "fieldset-legend", {login_label.clone()} }
                    EmailInput {
                        name: "email",
                        placeholder: "mail@site.com",
                    }
                    PasswordInput {
                        name: "password",
                        placeholder: tid!("frm-password"),
                        title: tid!("frm-password.err"),
                    }
                    button { class: "btn btn-neutral mt-4",
                        r#type: "submit",
                        { login_label }
                    }
                }
            }
        }
    }
}
