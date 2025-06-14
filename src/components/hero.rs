use dioxus::prelude::*;
use dioxus_i18n::tid;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            // Attributes should be defined in the element before any children
            id: "hero",
            // After all attributes are defined, we can define child elements and components
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", {tid!("hero.learn")} }
                a { href: "https://dioxuslabs.com/awesome", {tid!("hero.awesome")} }
                a { href: "https://github.com/dioxus-community/", {tid!("hero.community")} }
                a { href: "https://github.com/DioxusLabs/sdk", {tid!("hero.kit")} }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", {tid!("hero.code")} }
                a { href: "https://discord.gg/XgGxMSkvUM", {tid!("hero.discord")} }
            }
        }
    }
}
