use crate::{
    app::{DarkMode, Route},
    components::AlertDisplay,
    views::navbar::NavBar,
};
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn MainLayout() -> Element {
    let mode = use_context::<DarkMode>();
    rsx! {
        div {
            id: "main-foo",
            "data-theme": mode.theme(),
            NavBar {  }

            div {
                class: "container mx-auto px-4",
                AlertDisplay {}
                ErrorBoundary {
                    handle_error: move |error: ErrorContext| {
                        if let Some(error_ui) = error.show() {
                            rsx! {
                                {error_ui}
                            }
                        } else {
                            // alert.alert.set(Some((Alert::Error, tid!("unexpected"))));
                            rsx! {{tid!("unexpected")}}
                        }
                    },
                    Outlet::<Route> {}
                }
            }
        }
    }
}
