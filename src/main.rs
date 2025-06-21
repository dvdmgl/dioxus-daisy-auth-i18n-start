mod app;
mod components;
mod i18n;
mod shared;
mod views;

#[cfg(feature = "server")]
mod backend;

fn main() {
    #[cfg(feature = "web")]
    // Hydrate the application on the client
    dioxus::launch(app::App);

    // Launch axum on the server
    #[cfg(feature = "server")]
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                backend::launch_server(app::App).await;
            });
    }
}
