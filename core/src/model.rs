use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Envelope {
    pub protocol: String,
    pub version: u8,
    pub encoding: String,
    pub pk_sender: String,
    pub pk_recipient: String,
    pub issued_at: u64,
    pub sig: Option<String>,
    pub payload: String,
}
