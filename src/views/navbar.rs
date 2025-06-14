use crate::{
    app::{DarkMode, Route},
    i18n::LanguageSelect,
};
use dioxus::prelude::*;
use dioxus_i18n::tid;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    let mode = use_context::<DarkMode>();
    rsx! {
        div {
            id: "main-foo",
            "data-theme": mode.theme(),
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
                    ThemeControl {  }
                    LanguageSelect {}
                    button { class: "btn btn-ghost btn-circle",
                        div { class: "indicator",
                            svg {
                                class: "h-5 w-5",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                }
                            }
                            span { class: "badge badge-xs badge-primary indicator-item" }
                        }
                    }
                }
            }

            // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
            // the [`Home`] or [`Blog`] component depending on the current route.
            Outlet::<Route> {}
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
                    class: "swap-off h-6 w-6 fill-current",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" }
                }
                svg {
                    class: "swap-on h-6 w-6 fill-current",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
                }
            }
        }
    }
}
