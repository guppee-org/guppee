use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::extract::ws::{Message, WebSocket};
use futures::{Sink, SinkExt, Stream, StreamExt};
use shared::{ClientMessage, ServerMessage};

use crate::{error::Error, Result};

/// TRAIT ALIAS
#[doc(hidden)]
pub trait Threaded: Unpin + Send + Sync + 'static {}
impl<T> Threaded for T where T: Unpin + Send + Sync + 'static {}

/// TRAIT ALIAS
#[doc(hidden)]
pub trait ThreadedSink: Threaded + Sink<Message, Error = axum::Error> {}
impl<T> ThreadedSink for T where T: Threaded + Sink<Message, Error = axum::Error> {}

/// TRAIT ALIAS
#[doc(hidden)]
pub trait ThreadedStream: Threaded + Stream<Item = Result<Message, axum::Error>> {}
impl<T> ThreadedStream for T where T: Threaded + Stream<Item = Result<Message, axum::Error>> {}

pub struct Socket {
    receiver: Box<dyn ThreadedStream>,
    sender: Box<dyn ThreadedSink>,
}

impl std::fmt::Debug for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Socket")
            .field("receiver", &"Box(..)")
            .field("sender", &"Box(..)")
            .finish()
    }
}

impl From<WebSocket> for Socket {
    fn from(value: WebSocket) -> Self {
        let (sender, receiver) = value.split();
        Self {
            sender: Box::new(sender),
            receiver: Box::new(receiver),
        }
    }
}

impl Stream for Socket {
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

impl Sink<ServerMessage> for Socket {
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
