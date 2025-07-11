use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::{
    app::{AppGlobalState, Route},
    components::Alert,
    shared::user::{CheckEmail, RegisterPayload, check_user_is_free},
};

use super::components::{EmailInput, PasswordInput};

#[component]
pub fn Register() -> Element {
    let mut alert = use_context::<AppGlobalState>();
    let mut email = use_signal(String::new);
    let mut email_valid = use_signal(|| None);

    let update_email = move |evt: Event<FormData>| {
        email.set(evt.value());
        email_valid.set(None)
    };
    let nav = use_navigator();

    let form_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        let values = evt.values();
        let payload = RegisterPayload {
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
            let resp = crate::shared::user::submit_create_user(payload).await;
            match resp {
                Ok(Some(user)) => {
                    tracing::info!("Created {:?}", &user);
                    alert.alert.set(Some((
                        Alert::Success,
                        tid!("register.suc", username: user.email.clone()),
                    )));
                    nav.push(Route::Login {});
                }
                Err(e) => {
                    tracing::info!("the error {}", &e);
                    alert.alert.set(Some((Alert::Error, e.to_string())));
                }
                Ok(None) => {
                    tracing::info!("Nothing in the response");
                }
            }
        }
    };

    let check_if_valid = move |_: Event<_>| async move {
        if email().len() > 6 {
            let exists = check_user_is_free(CheckEmail {
                email: email().clone(),
            })
            .await;
            match exists {
                Ok(Some(false)) => {
                    email_valid.set(Some(("status-warning", tid!("frm-email.in-use"))))
                }
                Ok(Some(true)) => email_valid.set(Some(("status-success", tid!("frm-email.free")))),
                Err(_) | Ok(None) => email_valid.set(Some(("status-error", tid!("unexpected")))),
            };
        }
    };
    let register_label = tid!("register");

    rsx! {
        div {
            class: "flex justify-center items-center min-h-screen",
            form {
                // a fix for bug [prevent_default()](https://github.com/DioxusLabs/dioxus/issues/4303)
                action: "#",
                method: "dialog",
                onsubmit: form_submit,
                fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4",
                    legend { class: "fieldset-legend", {register_label.clone()} }
                    EmailInput {
                        name: "email",
                        placeholder: "mail@site.com",
                        onblur: check_if_valid,
                        oninput: update_email,
                    }
                    if let Some((status, msg)) = email_valid() {
                        div { class: "validator",
                            div { class: "status {status}",
                                aria_label: status
                            } {}
                            {msg}
                        }
                    } else {
                        div { class: "validator-hint", {tid!("frm-email.err")} }
                    }
                    PasswordInput {
                        name: "password",
                        placeholder: tid!("frm-password"),
                        title: tid!("frm-password.err"),
                    }
                    button { class: "btn btn-neutral mt-4",
                        { register_label }
                    }
                }
            }
        }
    }
}
