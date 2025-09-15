use serde::{Deserialize, Serialize};


// Se definen los tipos de mensajes
#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Connect,
    Disconnect,
    Text,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub msg_type: MessageType,
    pub user: String,
    pub content: String,
    pub timestamp: i64,
}