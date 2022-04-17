use serde_derive::{Serialize, Deserialize};
use hpke::{
    aead::{AeadTag, ChaCha20Poly1305},
    kdf::HkdfSha384,
    kem::X25519HkdfSha256,
    Kem as KemTrait, OpModeS, Serializable, Deserializable
};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    TriggerToTAP,
    TAPtoTrigger,
    TAPtoAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub msgtype: MessageType,
    pub contents: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransmissionData {
    pub encapped_key: Vec<u8>,
    pub cyphertext: Vec<u8>,
}
