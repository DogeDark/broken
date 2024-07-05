use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum SocketMessage {
    CompileRequest(String),
    CompileFinished(Result<Uuid, String>),
    CompileMessage(String),
}

impl SocketMessage {
    pub fn as_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl TryFrom<String> for SocketMessage {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}

#[cfg(feature = "web")]
use gloo_net::websocket::Message;

#[cfg(feature = "web")]
impl TryFrom<SocketMessage> for Message {
    type Error = serde_json::Error;

    fn try_from(value: SocketMessage) -> Result<Self, Self::Error> {
        let val = value.as_json_string()?;
        Ok(Self::Text(val))
    }
}

#[cfg(feature = "web")]
impl TryFrom<Message> for SocketMessage {
    type Error = serde_json::Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        match value {
            Message::Bytes(_) => unimplemented!(),
            Message::Text(txt) => Self::try_from(txt),
        }
    }
}
