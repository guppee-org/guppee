mod actor;
mod error;

use std::future::IntoFuture;

use axum::{routing::get, Json, Router};
use shared::PlayerId;
use tokio::{net::TcpListener, select};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{info, instrument, trace, Level};

use crate::actor::{Actor, Handle};

/// Result alias for [`error::Error`].
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

/// The directory being served by the server.
pub const DIST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../dist");

/// The address being binded to by the server.
pub const ADDR: &str = "0.0.0.0:8081";

/// Shared server state.
#[derive(Clone, Debug)]
struct App {
    /// Communication channel with [`Actor`].
    handle: crate::actor::Handle,
}

/// Start the http serve, serving the [`DIST`] directory.
#[instrument(level = Level::TRACE, ret)]
pub async fn start() -> Result<()> {
    let handle = Actor::start();

    let router = Router::new()
        .route("/ws", get(websocket_handler))
        .route("/players", get(player_list))
        .nest_service("/", ServeDir::new(DIST))
        .with_state(App { handle })
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

#[instrument(skip_all)]
async fn player_list(handle: Handle) -> Result<Json<Vec<PlayerId>>> {
    let list = handle.players().await?;
    Ok(Json(list))
}

#[instrument(skip_all, fields(registration))]
pub async fn websocket_handler(
    handle: Handle,
    ws: axum::extract::WebSocketUpgrade,
) -> axum::response::Response {
    ws.on_upgrade(move |ws| async move {
        let registration = handle.register(ws).await;
        tracing::Span::current().record("registration", format!("{registration:?}"));
    })
}
