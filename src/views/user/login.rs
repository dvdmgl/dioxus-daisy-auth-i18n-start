use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::{
    app::MyState,
    components::Alert,
    shared::user::{Credentials, User},
};

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut alert = use_context::<MyState>();

    let update_email = move |evt: Event<FormData>| {
        email.set(evt.value());
    };
    let update_password = move |evt: Event<FormData>| password.set(evt.value());
    let mut logged = use_context::<Signal<Option<User>>>();
    let nav = use_navigator();
    let login_label = tid!("login");
    rsx! {
        div {
            class: "flex justify-center items-center min-h-screen",
            form {
                onsubmit: move |evt| evt.prevent_default(),
                fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4",
                    legend { class: "fieldset-legend", {login_label.clone()} }
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
                        }
                    }
                    div { class: "validator-hint", {tid!("frm-email.err")} }
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
                            tracing::debug!("sending to server");
                            async move {
                                let payload  = Credentials {
                                    email: email.read().clone(),
                                    password: password.read().clone(),
                                    next: None
                                };
                                let resp = crate::shared::user::login_user(payload)
                                    .await;
                                match resp {
                                    Ok(Some(user)) =>  {
                                        logged.set(Some(user.clone()));
                                        alert.alert.set(Some((Alert::Info, tid!("login.suc", username: user.email))));
                                        nav.push("/");
                                    },
                                    Err(e) => {
                                        dbg!(&e);
                                        alert.alert.set(Some((Alert::Error, e.to_string())));
                                        // alert.alert.set(Some((Alert::Error, "motherf".into())));
                                    },
                                    Ok(None) => {
                                        tracing::info!("response has nothing");
                                    },
                                }
                            }
                        },
                        { login_label }
                    }
                }
            }
        }
    }
}
