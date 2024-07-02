use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    Sink, SinkExt, Stream, StreamExt,
};
use shared::{ClientMessage, ServerMessage};

use crate::{error::Error, Result};

/// Generic wrapper over an async stream to allow testing without a websocket
/// connection.
pub struct Socket<Tx = SplitSink<WebSocket, Message>, Rx = SplitStream<WebSocket>> {
    sender: Tx,
    receiver: Rx,
}

impl<Tx, Rx> Stream for Socket<Tx, Rx>
where
    Tx: Unpin,
    Rx: Stream<Item = Result<Message, axum::Error>> + Unpin,
{
    type Item = Result<ClientMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_next_unpin(cx).map(|maybe_try_msg| {
            let try_message = maybe_try_msg?;
            let coerced = try_message.map_err(Error::from);
            let deserialized = coerced.and_then(|message| {
                let text = message.into_text()?;
                Ok(ClientMessage::from_json(&text)?)
            });
            Some(deserialized)
        })
    }
}

impl<Tx, Rx> Sink<ServerMessage> for Socket<Tx, Rx>
where
    Tx: Sink<Message> + Unpin,
    Rx: Unpin,
    Tx::Error: Into<Error>,
{
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.sender.poll_ready_unpin(cx).map_err(Into::into)
    }

    fn start_send(mut self: Pin<&mut Self>, item: ServerMessage) -> Result<(), Self::Error> {
        self.sender
            .start_send_unpin(item.to_json()?.into())
            .map_err(Into::into)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.sender.poll_flush_unpin(cx).map_err(Into::into)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.sender.poll_close_unpin(cx).map_err(Into::into)
    }
}
