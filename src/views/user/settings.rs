use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::{
    app::{MyState, Route},
    components::Alert,
    shared::user::{ChangePassword, User},
};

use super::components::PasswordInput;

#[component]
pub fn UpdatePassword() -> Element {
    let navigator = use_navigator();
    let mut alert = use_context::<MyState>();
    let form_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        let values = evt.values();
        let payload = ChangePassword {
            old_password: values
                .get("old_password")
                .and_then(|v| v.first())
                .cloned()
                .unwrap_or_default(),
            new_password: values
                .get("old_password")
                .and_then(|v| v.first())
                .cloned()
                .unwrap_or_default(),
        };
        async move {
            let resp = crate::shared::user::change_password(payload).await;
            match resp {
                Ok(_) => {
                    alert
                        .alert
                        .set(Some((Alert::Info, tid!("frm-pass.suc-change"))));
                    navigator.push(Route::UserSettingsResume {});
                }
                Err(e) => {
                    alert.alert.set(Some((Alert::Error, e.to_string())));
                }
            }
        }
    };

    let label = tid!("frm-password.change");
    rsx! {
        div { class: "card bg-base-200 text-primary-content w-96",
            form {
                // a fix for bug [prevent_default()](https://github.com/DioxusLabs/dioxus/issues/4303)
                action: "#",
                method: "dialog",
                class: "flex justify-center items-center",
                onsubmit: form_submit,
                fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4",
                    legend { class: "fieldset-legend card-title", {label.clone()} }
                    PasswordInput {
                        name: "old_password",
                        placeholder: tid!("frm-password.old"),
                        title: tid!("frm-password.err"),
                    }
                    PasswordInput {
                        name: "new_password",
                        placeholder: tid!("frm-password.new"),
                        title: tid!("frm-password.err"),
                    }
                    button { class: "btn btn-neutral mt-4",
                        r#type: "submit",
                        { label }
                    }
                }
            }
        }
    }
}

#[component]
pub fn UserSettingsResume() -> Element {
    let mut auth = use_context::<Signal<Option<User>>>();
    let mut alert = use_context::<MyState>();
    let nav = use_navigator();
    let logout = move |_: Event<_>| async move {
        let _ = crate::shared::user::user_session_logout().await;
        auth.set(None);
        alert.alert.set(Some((Alert::Info, tid!("logout.suc"))));
        nav.push("/");
    };

    match auth() {
        Some(user) => rsx! {
            div { class: "card bg-primary text-primary-content w-96",
                div { class: "card-body",
                    h2 { class: "card-title", {user.email} }
                    p {
                        "A card component has a figure, a body part, and inside body thereare title and actions parts"
                    }
                    div { class: "card-actions justify-end",
                        button { class: "btn", onclick: logout, {tid!("logout")} }
                    }
                }
            }
        },
        None => {
            alert.alert.set(Some((Alert::Error, tid!("forbidden"))));
            rsx!()
        }
    }
}

#[component]
pub fn UserSettings() -> Element {
    let path: Route = use_route();
    rsx! {
        nav { class: "tabs tabs-border", role: "tablist",
            Link {
                class: if matches!(path, Route::UserSettingsResume { .. }) {
                    "tab tab-active"
                } else {
                    "tab"
                },
                role: "tab",
                to: Route::UserSettingsResume {  },
                "Resume"
            }
            Link {
                class: if matches!(path, Route::UpdatePassword { .. }) {
                    "tab tab-active"
                } else {
                    "tab"
                },
                role: "tab",
                to: Route::UpdatePassword {  },
                {tid!("frm-password.change")}
            }
        }
        Outlet::<Route> {}
    }
}
