use dioxus::prelude::*;

#[component]
pub fn PasswordInput(name: &'static str, placeholder: String, title: String) -> Element {
    rsx! {
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
                placeholder: "{placeholder}",
                r#type: "password",
                name: "{name}",
                required: true,
                title: "{title}",
            }
            p { class: "validator-hint hidden",
                "{title}"
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct EmailInputProps {
    name: &'static str,
    placeholder: String,
    onblur: Option<EventHandler<FocusEvent>>,
    oninput: Option<EventHandler<FormEvent>>,
}

#[allow(non_snake_case)]
pub fn EmailInput(props: EmailInputProps) -> Element {
    rsx! {
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
                placeholder: props.placeholder,
                name: props.name,
                r#type: "email",
                required: true,
                onblur: move |evt| {
                    if let Some(handler) = &props.onblur {
                        handler.call(evt);
                    }
                },
                oninput: move |evt| {
                    if let Some(handler) = &props.oninput {
                        handler.call(evt);
                    }
                }
            }
        }
    }
}
