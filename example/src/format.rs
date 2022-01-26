pub enum MessageType {
    Type1,
    Type2,
}

pub struct Message {
    pub msgtype: MessageType,
    pub contents: Vec<u8>,
}
