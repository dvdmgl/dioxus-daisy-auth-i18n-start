use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::app::MyState;

pub enum Alert {
    Info,
    Success,
    Warning,
    Error,
}

impl Alert {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Info => "alert-info",
            Self::Success => "alert-success",
            Self::Warning => "alert-warning",
            Self::Error => "alert-error",
        }
    }
    pub fn path(&self) -> &'static str {
        match self {
            Self::Info => "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
            Self::Success => "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
            Self::Warning => {
                "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
            }
            Self::Error => "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
        }
    }
}

#[component]
pub fn AlertDisplay() -> Element {
    let mut alert_msg = use_context::<MyState>().alert;
    let close = move |_: Event<_>| alert_msg.set(None);
    if let Some((alert, msg)) = &*alert_msg.read() {
        let message = if let Some(m) = msg.strip_prefix("error running server function: ") {
            tid!(m)
        } else {
            msg.clone()
        };
        rsx! {
            div {
                role: "alert",
                class: "alert {alert.class()} alert-vertical sm:alert-horizontal",
                svg {
                    class: "h-6 w-6 shrink-0 stroke-current",
                    fill: "none",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "{alert.path()}"
                    }
                }
                span { {message} }
                div {
                    button {
                        class: "btn btn-sm btn-primary",
                        onclick: close,
                        {tid!("bu.close")}
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
