use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Type1,
    Type2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub msgtype: MessageType,
    pub contents: Vec<u8>,
}
