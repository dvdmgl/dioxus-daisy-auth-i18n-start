use dioxus::prelude::*;
pub mod user;

#[server(EchoServer)]
pub async fn echo_server(input: String) -> Result<String, ServerFnError> {
    let auth: axum::Extension<crate::backend::BackendState> = extract().await?;
    let pg = auth.0.db.get().await?;
    tracing::debug!("echo {}", &input);

    dbg!(&pg.query("SELECT 1", &[]).await?);
    // The body of server function like this comment are only included on the server. If you have any server-only logic like
    // database queries, you can put it here. Any imports for the server function should either be imported inside the function
    // or imported under a `#[cfg(feature = "server")]` block.
    Ok(input)
}
