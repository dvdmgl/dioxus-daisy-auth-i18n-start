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
        #[nest("/settings")]
            #[layout(UserSettings)]
                #[route("/")]
                UserSettingsResume {},
                #[route("/password")]
                UpdatePassword {},
        //     #[end_layout]
        // #[end_nest]
    // #[end_layout]
}

#[derive(Clone, Copy, Default)]
pub struct MyState {
    pub alert: Signal<Option<(Alert, String)>>,
}

// handling FOUD, if the store doesn't have `data-theme` will be set the window.matchMedia,
// components::ThemeControl will get handle it's changes and state by using setAttribute and localStorage.setItem
static THEME_BOOTSTRAP: &'static str = r#"
    <script>
    (function() {
        try {
            const savedTheme = localStorage.getItem('data-theme');
            const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
            if (savedTheme !== null) {
                document.documentElement.setAttribute('data-theme', savedTheme);
            } else if (prefersDark) {
                localStorage.setItem('data-theme', 'dark');
                document.documentElement.setAttribute('data-theme', 'dark');
            } else {
                localStorage.setItem('data-theme', 'light');
                document.documentElement.setAttribute('data-theme', 'light');
            }
            // Apply the theme attribute to the root HTML element
            // console.log("data-theme", theme);
        } catch (e) {
            // Log any errors without disrupting the page load
            console.error("Theme pre-render script error:", e);
        }
    })();
    </script>
"#;

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
pub fn App() -> Element {
    // Initialize logged_user from the server
    let user = use_server_future(crate::shared::user::get_user_session)?
        .clone()
        .unwrap()
        .map_err(CapturedError::from_display)?;
    tracing::debug!("is there a logged user {:?}", user);
    use_context_provider(|| Signal::new(user));

    use_context_provider(MyState::default);
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
        div {
            dangerous_inner_html: "{THEME_BOOTSTRAP}",
            Router::<Route> {}
        }
    }
}
