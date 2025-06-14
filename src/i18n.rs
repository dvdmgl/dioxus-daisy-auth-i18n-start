use dioxus::{logger::tracing::debug, prelude::*};
use dioxus_i18n::{
    prelude::i18n,
    tid,
    unic_langid::{langid, LanguageIdentifier},
};

pub static EN_US: LanguageIdentifier = langid!("en-US");
pub static PT_PT: LanguageIdentifier = langid!("pt-PT");

#[component]
pub fn LanguageSelect() -> Element {
    let mut i18n = i18n();
    let active_lang = i18n.language();
    debug!("Changed language to {:?}", &active_lang);
    rsx! {
        div { class: "dropdown dropdown-end mr-2",
            label {
                tabindex: "0",
                class: "btn btn-ghost",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "h-5 w-5",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M3 5h12M3 9h12M3 13h12M3 17h12M15 5l6 6M15 9l6 6M15 13l6 6M15 17l6 6",
                    }
                }
                span { class: "hidden md:inline", {tid!("navbar.language-select")} }
            }
            ul {
                tabindex: "0",
                class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52",
                li {
                    input {
                        r#type: "radio",
                        name: "lang",
                        class: "theme-controller btn btn-sm btn-block justify-start",
                        aria_label: "English",
                        value: EN_US.to_string(),
                        checked: active_lang == EN_US,
                        onchange: move |_| {i18n.set_language(EN_US.clone());},
                    }
                }
                li {
                    input {
                        r#type: "radio",
                        name: "lang",
                        class: "theme-controller btn btn-sm btn-block justify-start",
                        aria_label: "Português",
                        value: PT_PT.to_string(),
                        checked: active_lang == PT_PT,
                        onchange: move |_| {i18n.set_language(PT_PT.clone());},
                    }
                }
            }
        }
    }
}
