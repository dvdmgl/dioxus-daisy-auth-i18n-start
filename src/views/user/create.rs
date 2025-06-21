use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::{
    app::MyState,
    components::Alert,
    shared::user::{CheckEmail, RegisterPayload, check_user_is_free},
};

#[component]
pub fn Register() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    // let mut resp_user = use_signal(|| None);
    let mut email_Valid = use_signal(|| None);

    let update_email = move |evt: Event<FormData>| {
        email.set(evt.value());
        email_Valid.set(None)
    };
    let update_password = move |evt: Event<FormData>| password.set(evt.value());
    let check_if_valid = move |_: Event<_>| async move {
        if email().len() > 6 {
            let exists = check_user_is_free(CheckEmail {
                email: email().clone(),
            })
            .await;
            match exists {
                Ok(Some(false)) => {
                    email_Valid.set(Some(("status-warning", tid!("frm-email.in-use"))))
                }
                Ok(Some(true)) => email_Valid.set(Some(("status-success", tid!("frm-email.free")))),
                Err(_) | Ok(None) => email_Valid.set(Some(("status-error", tid!("unexpected")))),
            };
        }
    };
    let register_label = tid!("register");
    let mut alert = use_context::<MyState>();
    rsx! {
        div {
            class: "flex justify-center items-center min-h-screen",
            form {
                onsubmit: move |evt| evt.prevent_default(),
                fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4",
                    legend { class: "fieldset-legend", {register_label.clone()} }
                    label {
                        class: "input validator",
                        svg {
                            class: "h-[1em] opacity-50",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            g {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2.5",
                                rect {
                                    height: "16",
                                    rx: "2",
                                    width: "20",
                                    x: "2",
                                    y: "4",
                                }
                                path { d: "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" }
                            }
                        }
                        input {
                            placeholder: "mail@site.com",
                            r#type: "email",
                            required: "false",
                            oninput: update_email,
                            onblur: check_if_valid
                        }
                    }
                    if let Some((status, msg)) = email_Valid() {
                        div { class: "validator",
                            div { class: "status {status}",
                                aria_label: status
                            } {}
                            {msg}
                        }
                    } else {
                        div { class: "validator-hint", {tid!("frm-email.err")} }
                    }
                    label {
                        class: "input validator",
                        svg {
                            class: "h-[1em] opacity-50",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            g {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2.5",
                                path { d: "M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z" }
                                circle {
                                    cx: "16.5",
                                    cy: "7.5",
                                    fill: "currentColor",
                                    r: ".5",
                                }
                            }
                        }
                        input {
                            minlength: "8",
                            placeholder: tid!("frm-password"),
                            r#type: "password",
                            required: "false",
                            title: tid!("frm-password.err"),
                            oninput: update_password,
                        }
                        p { class: "validator-hint hidden",
                            {tid!("frm-password.err")}
                        }
                    }
                    button { class: "btn btn-neutral mt-4",
                        onclick: move |e| {
                            e.prevent_default();
                            async move {
                                let payload  = RegisterPayload {
                                    email: email.read().clone(),
                                    password: password.read().clone(),
                                };
                                let resp = crate::shared::user::submit_create_user(payload)
                                    .await;
                                match resp {
                                    Ok(Some(user)) =>  {
                                        tracing::info!("Created {:?}", &user);
                                        alert.alert.set(Some((Alert::Success, tid!("register.suc", username: user.email.clone()))));
                                        // resp_user.set(Some(user.clone()));
                                    },
                                    Err(e) => {
                                        dbg!(&e);
                                        tracing::info!("the motherfucking error, basterd {}", &e);
                                    },
                                    Ok(None) => {
                                        tracing::info!("response has nothing");
                                        // resp_user.set(None);
                                    },
                                }
                            }
                        },
                        { register_label }
                    }
                }
            }
        }
    }
}
