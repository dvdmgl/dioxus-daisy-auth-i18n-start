use crate::{
    app::{MyState, Route},
    components::{Alert, ThemeControl},
    i18n::LanguageSelect,
    shared::user::LoggedUser,
};
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div {
            class: "navbar bg-base-100 shadow-sm",
            id: "navbar",
            div {
                class: "navbar-start",
                ul {
                    class: "menu menu-horizontal px-1",
                    li {
                        Link {
                            to: Route::Home {},
                            {tid!("navbar.home")}
                        }
                    }
                    li {
                        Link {
                            to: Route::Blog { id: 1 },
                            {tid!("navbar.blog")}
                        }
                    }
                }
            }
            div { class: "navbar-end",
                AuthOptions {}
                ThemeControl {}
                LanguageSelect {}
            }
        }
    }
}

#[component]
fn AuthOptions() -> Element {
    let mut auth = use_context::<Signal<Option<LoggedUser>>>();
    let mut alert = use_context::<MyState>();
    let nav = use_navigator();
    let logout = move |_: Event<_>| async move {
        let _ = crate::shared::user::user_session_logout().await;
        auth.set(None);
        alert.alert.set(Some((Alert::Info, tid!("logout.suc"))));
        nav.push("/");
    };
    if let Some(auth_acc) = auth() {
        rsx! {
            div { class: "dropdown dropdown-end",
                div {
                    class: "btn btn-ghost btn-circle avatar",
                    role: "button",
                    tabindex: "0",
                    div { class: "rounded-full",
                        svg {
                            class: "size-[1.2em]",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M17.982 18.725A7.488 7.488 0 0 0 12 15.75a7.488 7.488 0 0 0-5.982 2.975m11.963 0a9 9 0 1 0-11.963 0m11.963 0A8.966 8.966 0 0 1 12 21a8.966 8.966 0 0 1-5.982-2.275M15 9.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                            }
                        }
                    }
                }
                ul {
                    class: "menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow",
                    tabindex: "0",
                    li {
                        Link {
                            to: Route::UserSettingsResume {  },
                            {auth_acc.user.email}
                        }
                    }
                    li {
                        a { onclick: logout,
                            {tid!("logout")}
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            ul {
                class: "menu menu-horizontal px-1",
                li {
                    Link {
                        to: Route::Register {  },
                        {tid!("register")}
                    }
                }
                li {
                    Link {
                        class: "btn btn-ghost btn-circle",
                        to: Route::Login {  },
                        svg {
                            class: "size-[1.2em]",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M8.25 9V5.25A2.25 2.25 0 0 1 10.5 3h6a2.25 2.25 0 0 1 2.25 2.25v13.5A2.25 2.25 0 0 1 16.5 21h-6a2.25 2.25 0 0 1-2.25-2.25V15M12 9l3 3m0 0-3 3m3-3H2.25",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                            }
                        }
                    }
                }
            }
        }
    }
}
