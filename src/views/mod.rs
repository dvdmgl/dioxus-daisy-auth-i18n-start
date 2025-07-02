mod home;
pub use home::Home;

mod blog;
pub use blog::Blog;

mod navbar;
// pub use navbar::MainLayout;

mod layout;
pub use layout::MainLayout;

mod user;
pub use user::{
    create::Register,
    login::Login,
    settings::{UpdatePassword, UserSettings, UserSettingsResume},
};
