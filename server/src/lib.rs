use axum::routing::Router;
use thiserror::Error;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

const DIST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/dist");

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Runtime(#[from] tokio::io::Error),
    #[error(transparent)]
    Parse(#[from] tracing_subscriber::filter::ParseError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Start the http serve, serving the [`DIST`] directory.
#[tracing::instrument(ret)]
pub async fn start() -> Result<()> {
    let router = Router::new()
        .nest_service("/", ServeDir::new(DIST))
        .layer(TraceLayer::new_for_http());

    let tcp_listener = TcpListener::bind("0.0.0.0:8081").await?;
    tracing::debug!(tcp.addr = ?tcp_listener.local_addr());
    axum::serve(tcp_listener, router).await?;
    Ok(())
}
