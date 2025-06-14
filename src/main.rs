/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

mod app;
mod backend;
mod i18n;

fn main() {
    dioxus::launch(app::App);
}
