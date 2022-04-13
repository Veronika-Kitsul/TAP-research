use serde_derive::{Serialize, Deserialize};
use hpke::{
    aead::{AeadTag, ChaCha20Poly1305},
    kdf::HkdfSha384,
    kem::X25519HkdfSha256,
    Kem as KemTrait, OpModeS, Serializable, Deserializable
};

type Kem = X25519HkdfSha256;
type Aead = ChaCha20Poly1305;
type Kdf = HkdfSha384;


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

#[derive(Serialize, Deserialize, Debug)]
pub struct TransmissionData {
    pub encapped_key: Vec<u8>,
    pub cyphertext: Vec<u8>,
}
