#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserConfig {
    pub username: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Peer {
    pub name: String,
    pub pub_key_b64: String,
}

#[derive(Debug, Clone)]
pub enum MessageDirection {
    Incoming,
    Outgoing,
}

#[derive(Debug, Clone)]
pub struct PlainMessage {
    pub from: String,
    pub to: String,
    pub body: String,
    pub direction: MessageDirection,
}