mod ws;

use std::future::IntoFuture;

use axum::routing::{get, Router};
use thiserror::Error;
use tokio::{net::TcpListener, select};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{info, instrument, trace, Level};

/// Result alias for [`Error`].
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The directory being served by the server.
pub const DIST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../dist");

/// The address being binded to by the server.
pub const ADDR: &str = "0.0.0.0:8081";

/// [`crate`] error types.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Runtime(#[from] tokio::io::Error),
    #[error(transparent)]
    Parse(#[from] tracing_subscriber::filter::ParseError),
}

/// Start the http serve, serving the [`DIST`] directory.
#[instrument(level = Level::TRACE, ret)]
pub async fn start() -> Result<()> {
    let router = Router::new()
        .route("/ws", get(crate::ws::handler))
        .nest_service("/", ServeDir::new(DIST))
        .layer(TraceLayer::new_for_http());

    let tcp_listener = TcpListener::bind(ADDR).await?;

    info!(
        serve.address = ADDR,
        serve.directory = DIST,
        serve.url = format!("https://0.0.0.0:8081"),
    );

    let server_future = axum::serve(tcp_listener, router).into_future();
    select! {
        _ = tokio::signal::ctrl_c() => trace!(
            "received ctrl-c signal. shutting down."
        ),
        _ = server_future => trace!(
            "server future resolved, indicating a crash."
        ),
    }
    Ok(())
}
