use dioxus::prelude::*;
use dioxus_i18n::tid;

use super::components::{EmailInput, PasswordInput};
use crate::{
    app::MyState,
    components::Alert,
    shared::user::{Credentials, User},
};

#[component]
pub fn Login() -> Element {
    let mut alert = use_context::<MyState>();

    let mut logged = use_context::<Signal<Option<User>>>();
    let nav = use_navigator();

    let form_submit = move |evt: Event<FormData>| {
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
            next: None,
        };

        async move {
            let resp = crate::shared::user::login_user(payload.clone()).await;
            match resp {
                Ok(Some(user)) => {
                    logged.set(Some(user.clone()));
                    alert
                        .alert
                        .set(Some((Alert::Info, tid!("login.suc", username: user.email))));
                    nav.push("/");
                }
                Err(e) => {
                    alert.alert.set(Some((Alert::Error, e.to_string())));
                }
                Ok(None) => {
                    tracing::info!("response has nothing");
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
