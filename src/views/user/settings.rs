use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::{
    app::{AppGlobalState, Route},
    components::Alert,
    shared::user::{ChangePassword, LoggedUser},
};

use super::components::PasswordInput;

#[component]
pub fn UpdatePassword() -> Element {
    let navigator = use_navigator();
    let mut alert = use_context::<AppGlobalState>();
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
    let mut auth = use_context::<Signal<Option<LoggedUser>>>();
    let mut app_state = use_context::<AppGlobalState>();
    let nav = use_navigator();
    let logout = move |_: Event<_>| async move {
        let _ = crate::shared::user::user_session_logout().await;
        auth.set(None);
        app_state.alert.set(Some((Alert::Info, tid!("logout.suc"))));
        nav.push("/");
    };

    match auth() {
        Some(user) => {
            let role = user.user.role;
            let c_at = user.user.c_at.format("%Y-%m-%d %H:%M:%S");
            let u_at = user.user.u_at.format("%Y-%m-%d %H:%M:%S");
            rsx! {
                div { class: "card bg-base-200 text-primary-content w-96",
                    div { class: "card-body",
                        h2 { class: "card-title",
                            {user.user.email}
                            div { class:"badge badge-secondary",
                                "{role:?}"
                            }
                        }
                        ul { class: "list rounded-box shadow-md",
                            li { class: "list-row",
                                div {
                                    {tid!("date.c-at")}
                                }
                                span { "{c_at}" }
                            }
                            li { class: "list-row",
                                div {
                                    {tid!("date.u-at")}
                                }
                                span { "{u_at}" }
                            }
                            li { class: "p-4 pb-2 text-xs opacity-60 tracking-wide",
                                div {"permissions"}
                            }
                            {user.perms.iter().map(|perm| rsx! {
                                li { class: "list-row",
                                    span {"{perm:?}"}
                                }
                            })}
                        }
                        div { class: "card-actions justify-end",
                            button { class: "btn", onclick: logout, {tid!("logout")} }
                        }
                    }
                }
            }
        }
        None => {
            app_state.alert.set(Some((Alert::Error, tid!("forbidden"))));
            rsx!()
        }
    }
}

#[component]
pub fn UserSettings() -> Element {
    let auth = use_context::<Signal<Option<LoggedUser>>>();
    let mut app_state = use_context::<AppGlobalState>();
    let path: Route = use_route();
    let nav = use_navigator();

    match auth() {
        Some(_) => {
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
                        {tid!("resume")}
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
        None => {
            app_state.alert.set(Some((Alert::Error, tid!("forbidden"))));
            app_state.redirect.set(path.to_string());
            nav.push(Route::Login {});
            rsx!()
        }
    }
}
