# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # Entrypoint
│  ├─ app.rs # The entrypoint for the app. It also defines the routes for the app.
│  ├─ locales/ # translations folder
│  │  ├─ en-US.ftl
│  ├─ backend/ # server logic
│  │  ├─ mod.rs # Defines the backend module
│  │  ├─ errors.rs # BackendError
│  │  ├─ auth/ # server authentication logic/state
│  │  ├─ user.rs # server User logic/state
│  ├─ shared/
│  │  ├─ mod.rs # Defines the shared structs and functions
│  │  ├─ user.rs # Shared authentication and user models
│  ├─ components/
│  │  ├─ mod.rs # Defines the components module
│  │  ├─ alert.rs # The Alert component
│  │  ├─ hero.rs # The Hero component for use in the home page
│  │  ├─ echo.rs # The echo component uses server functions to communicate with the server
│  ├─ views/ # The views each route will render in the app.
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
│  │  ├─ layout.rs # The component that will render at the / route
│  │  ├─ navbar.rs # translation and theme select, login, logout
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### crates
dioxus requires axum@0.7, assert that all crates meet the axum@0.7 requirement.
- [axum-login@0.16](https://crates.io/crates/axum-login/0.16.0)
- [axum@0.7](https://crates.io/crates/axum/0.7.9)
- [dioxus-i18n@0.4](https://crates.io/crates/dioxus-i18n/0.4.3)

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

- [heroicons](https://heroicons.com/)
