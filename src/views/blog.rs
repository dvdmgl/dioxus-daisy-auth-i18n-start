use crate::app::Route;
use dioxus::prelude::*;
use dioxus_i18n::tid;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        div {
            id: "blog",

            // Content
            h1 { {tid!("blog", id: id)} }
            p { {tid!("blog.description", id: id)} }

            // Navigation links
            // The `Link` component lets us link to other routes inside our app. It takes a `to` prop of type `Route` and
            // any number of child nodes.
            Link {
                // The `to` prop is the route that the link should navigate to. We can use the `Route` enum to link to the
                // blog page with the id of -1. Since we are using an enum instead of a string, all of the routes will be checked
                // at compile time to make sure they are valid.
                to: Route::Blog { id: id - 1 },
                {tid!("bu.prev")}
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                {tid!("bu.next")}
            }
        }
    }
}
