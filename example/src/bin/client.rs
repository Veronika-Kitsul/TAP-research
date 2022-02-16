use std::io::{Read, Write};
use std::net::TcpStream;
use example::format::{Message,MessageType};
use bincode;
use rand::prelude::*;
use hpke::{
    aead::{AeadTag, ChaCha20Poly1305},
    kdf::HkdfSha384,
    kem::X25519HkdfSha256,
    Deserializable, Kem as KemTrait, OpModeR, OpModeS, Serializable,
};

type Kem = X25519HkdfSha256;
type Aead = ChaCha20Poly1305;
type Kdf = HkdfSha384;

const INFO_STR: &[u8] = b"example session";

// initialize the server with a key pair
fn server_init() -> (<Kem as KemTrait>::PrivateKey, <Kem as KemTrait>::PublicKey) {
    let mut csprng = StdRng::from_entropy();
    Kem::gen_keypair(&mut csprng)
}

fn encrypt_msg(
    msg: &[u8],
    aad: &[u8],
    server_pk: &<Kem as KemTrait>::PublicKey,) -> (<Kem as KemTrait>::EncappedKey, Vec<u8>, AeadTag<Aead>) {
        let mut csprng = StdRng::from_entropy();

        // encapsulate a key and use the resulting shared secret to encrypt a message
        // encrypt with AEAD context
        let (encapsulated_key, mut sender_ctx) = hpke::setup_sender::<Aead, Kdf, Kem, _>
                                                    (&OpModeS::Base, &server_pk, INFO_STR, 
                                                    &mut csprng).expect("invalid server pubkey!");
        // seal in place will encrypt the plaintext in place if success
        let mut msg_copy = msg.to_vec();
        let tag = sender_ctx.seal_in_place_detached(&mut msg_copy, aad).expect("encryption failed!");

        let ciphertext = msg_copy;
        println!("ciphertext: {:?}", ciphertext);
        // return
        (encapsulated_key, ciphertext, tag)
    }


fn main() {
    // set up the server 
    let (server_privkey, server_pubkey) = server_init();

     // Create a message
    let msg = Message {
        msgtype: MessageType::Type1,
        contents: b"Hello".to_vec(),
    };

    let aad = b"First encrypted message";

    // Marshall the message into bincode
    let serialized: Vec<u8> = bincode::serialize(&msg).unwrap();
    // looks good
    println!("serialized: {:?}",serialized);

    let (encapsulated_key, ciphertext, tag) = encrypt_msg(&serialized, aad, &server_pubkey);
    let encapsulated_key_bytes = encapsulated_key.to_bytes();
    let tag_bytes = tag.to_bytes();

    // Open a connection to the server
    match TcpStream::connect("127.0.0.1") {

        Ok(mut stream) => {
            println!("Connected to the server!");

            // Send ciphertext to the server
            stream.write(&ciphertext).unwrap();

            println!("Awaiting reply");

            let mut data = [0 as u8; 2000];
            match stream.read_exact(&mut data) {
                Ok(data) => {
                    println!("{:?}", data)
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
