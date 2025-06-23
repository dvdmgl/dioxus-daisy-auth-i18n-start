use crate::{app::Route, components::AlertDisplay, views::navbar::NavBar};
use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn MainLayout() -> Element {
    rsx! {
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
                        rsx! {{tid!("unexpected")}}
                    }
                },
                Outlet::<Route> {}
            }
        }
    }
}
