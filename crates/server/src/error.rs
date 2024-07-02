use axum::{extract::ws::WebSocket, Json};
use tokio::sync::{mpsc, oneshot};
use tracing::error;
use tracing_subscriber::filter;

/// [`crate`] error types.
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    Runtime(#[from] tokio::io::Error),
    Parse(#[from] filter::ParseError),
    Register(#[from] mpsc::error::SendError<WebSocket>),
    Oneshot(#[from] oneshot::error::RecvError),
    Serialization(#[from] serde_json::Error),
    Server(#[from] axum::Error),
    Erased(Box<dyn std::error::Error + Send + Sync>),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        // Log the error on a failed response and still notify with a response
        // over http instead of ws.
        error!(server.error = self.to_string());
        Json(self.to_string()).into_response()
    }
}

impl Error {
    pub fn erased(e: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Erased(Box::new(e))
    }
}
