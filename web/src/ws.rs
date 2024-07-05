use dioxus::signals::{Signal, Writable as _, WritableVecExt as _};
use futures::{SinkExt as _, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use model::*;

use crate::error::AppError;

pub struct Socket {
    socket: WebSocket,
}

impl Socket {
    pub fn new(url: &str) -> Result<Self, AppError> {
        let socket = WebSocket::open(url).map_err(|e| AppError::Socket(e.to_string()))?;

        Ok(Self { socket })
    }

    pub async fn compile(&mut self, code: String) -> Result<(), AppError> {
        self.socket
            .send(SocketMessage::CompileRequest(code).try_into()?)
            .await
            .map_err(|e| AppError::Socket(e.to_string()))
    }

    pub async fn next(&mut self) -> Option<SocketMessage> {
        match self.socket.next().await {
            Some(Ok(msg)) => SocketMessage::try_from(msg).ok(),
            _ => None,
        }
    }

    pub async fn close(self) {
        self.socket.close(None, None).ok();
    }
}

pub fn handle_message(
    mut is_compiling: Signal<bool>,
    mut built_page_id: Signal<Option<String>>,
    mut compiler_messages: Signal<Vec<String>>,
    msg: SocketMessage,
) -> bool {
    match msg {
        SocketMessage::CompileFinished(res) => {
            match res {
                Ok(id) => {
                    is_compiling.set(false);
                    built_page_id.set(Some(id.to_string()))
                }
                Err(e) => {
                    is_compiling.set(false);
                    built_page_id.set(None);
                    compiler_messages.push(format!("Error: {e}"));
                }
            }

            true
        }
        SocketMessage::CompileMessage(msg) => {
            compiler_messages.push(msg);
            false
        }
        _ => false,
    }
}
