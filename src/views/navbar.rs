use crate::{
    app::{DarkMode, MyState, Route},
    components::Alert,
    i18n::LanguageSelect,
    shared::user::User,
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
    let mut auth = use_context::<Signal<Option<User>>>();
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
                        a { class: "justify-between",
                            "auth_acc.email"
                            span { class: "badge", "New" }
                        }
                    }
                    li {
                        a { "Settings" }
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

#[component]
fn ThemeControl() -> Element {
    let mut mode = use_context::<DarkMode>();
    let is_dark = *mode.0.read();
    rsx! {
        button { class: "btn btn-ghost btn-circle",
            label { class: "swap swap-rotate",
                input {
                    class: "theme-controller",
                    r#type: "checkbox",
                    value: "dark",
                    onchange: move |_| mode.0.set(!is_dark),
                }
                svg {
                    class: "swap-off size-[1.2em] fill-current",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" }
                }
                svg {
                    class: "swap-on size-[1.2em] fill-current",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
                }
            }
        }
    }
}
