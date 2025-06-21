use crate::{components::Alert, views::*};
use dioxus::{CapturedError, prelude::*};
use dioxus_i18n::prelude::{I18nConfig, Locale, use_init_i18n};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(MainLayout)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Home {},
        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] with the `:` syntax.
        // In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        #[route("/blog/:id")]
        // Fields of the route variant will be passed to the component as props. In this case, the blog component must accept
        // an `id` prop of type `i32`.
        Blog { id: i32 },
        #[route("/register")]
        Register {},
        #[route("/login")]
        Login {},
}

#[derive(Clone)]
pub struct DarkMode(pub Signal<bool>);

impl DarkMode {
    pub fn theme(&self) -> &'static str {
        if *self.0.read() { "dark" } else { "light" }
    }
}

#[derive(Clone, Copy, Default)]
pub struct MyState {
    pub alert: Signal<Option<(Alert, String)>>,
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
pub fn App() -> Element {
    let is_dark = use_signal(|| false);

    // Initialize logged_user from the server
    let user = use_server_future(crate::shared::user::get_user_session)?
        .clone()
        .unwrap()
        .map_err(CapturedError::from_display)?;

    use_context_provider(MyState::default);
    use_context_provider(|| DarkMode(is_dark));
    use_context_provider(|| Signal::new(user));
    use_init_i18n(|| {
        I18nConfig::new(crate::i18n::EN_US.clone())
            .with_locale(Locale::new_static(
                crate::i18n::EN_US.clone(),
                include_str!("./locales/en-US.ftl"),
            ))
            .with_locale(Locale::new_static(
                crate::i18n::PT_PT.clone(),
                include_str!("./locales/pt-PT.ftl"),
            ))
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
