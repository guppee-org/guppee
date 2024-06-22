use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use tracing::{error, info, instrument};

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub(crate) enum Error {
    Axum(#[from] axum::Error),
}

#[instrument(skip_all)]
pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(upgrade)
}

#[instrument(skip_all)]
async fn upgrade(mut socket: WebSocket) {
    while let Some(raw_message) = socket.recv().await {
        let message = raw_message.map_err(Error::from);
        if let Err(err) = recv_message(message).await {
            error!(?err);
        }
    }
}

#[instrument(skip_all)]
async fn recv_message(message: Result<Message>) -> Result<()> {
    let message = message?;
    let text = message.into_text()?;
    info!(ws.message = text);
    Ok(())
}
