use std::io::{Read, Write};
use std::net::TcpStream;
use example::format::{Message,MessageType,TransmissionData};
use bincode;
use std::io::BufReader;
use std::fs::File;
use rand::prelude::*;
use hpke::{
    aead::{AeadTag, ChaCha20Poly1305},
    kdf::HkdfSha384,
    kem::X25519HkdfSha256,
    Kem as KemTrait, OpModeS, Serializable, Deserializable
};

// WHAT ARE THOSE ??? and how do we pick those?
// more resources on the types of them
type Kem = X25519HkdfSha256;
type Aead = ChaCha20Poly1305;
type Kdf = HkdfSha384;

// WHAT IS THIS AND WHY DO WE NEED IT AND DO WE NEED TO CHANGE IT DEPENDING ON THE SESSION ??????
const INFO_STR: &[u8] = b"session";

fn encrypt_msg(
    // passed the serialized message
    msg: &[u8],
    aad: &[u8],
    pub_key: &<Kem as KemTrait>::PublicKey,) -> TransmissionData {
         let mut csprng = StdRng::from_entropy();

         let (encapsulated_key, mut encryption_context) =
        hpke::setup_sender::<Aead, Kdf, Kem, _>(&OpModeS::Base, &pub_key, INFO_STR, &mut csprng)
        .expect("invalid server pubkey!");
        
        // seal in place will encrypt the plaintext in place if success
        let mut msg_copy = msg.to_vec();

        let ciphertext = encryption_context.seal(msg, aad).expect("encryption failed!");

        let encapsulated_key_vec = encapsulated_key.to_bytes().to_vec();

        TransmissionData{encapped_key: encapsulated_key_vec, cyphertext: ciphertext}
    }


fn main() {
     // public key retrieval
        // set up the server 
        let f = File::open("public.txt").unwrap();
        let mut reader = BufReader::new(f);
        let mut pub_key_bytes = Vec::new();
        
        // Read file into vector.
        reader.read_to_end(&mut pub_key_bytes).unwrap();

        // deserialize into private key type from the file
        let pub_key: <Kem as KemTrait>::PublicKey =
            Deserializable::from_bytes(&pub_key_bytes).unwrap();

     // Create a message
     // MESSAGE CONTENTS HOW DO WE GET HERE ????
    let msg = Message {
        msgtype: MessageType::Type1,
        contents: b"Hello".to_vec(),
    };

    // do we need to generate aad as well so it's more secure this was?
    let aad = b"";

    // Marshall the message into bincode
    let serialized: Vec<u8> = bincode::serialize(&msg).unwrap();

    // symmetric 
    let data = encrypt_msg(&serialized, aad, &pub_key);
    let data_serialized: Vec<u8> = bincode::serialize(&data).unwrap();

    // Open a connection to the server
    match TcpStream::connect("127.0.0.1:8080") {

        Ok(mut stream) => {
            println!("Connected to the server!");
            stream.write(&data_serialized).unwrap();

            let mut data = [0 as u8; 5000];

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
