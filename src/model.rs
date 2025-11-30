
#[derive(Debug, Clone)]
pub struct UserConfig {
    pub username: String,
    // pub key_id: String,
}

#[derive(Debug, Clone)]
pub struct Peer {
    pub name: String,
    pub key_id: String,
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
