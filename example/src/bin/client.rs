use std::io::{Read, Write};
use std::net::TcpStream;
use example::format::{Message,MessageType};
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
type Kem = X25519HkdfSha256;
type Aead = ChaCha20Poly1305;
type Kdf = HkdfSha384;

// WHAT IS THIS AND WHY DO WE NEED IT AND DO WE NEED TO CHANGE IT DEPENDING ON THE SESSION ??????
const INFO_STR: &[u8] = b"example session";

fn encrypt_msg(
    // use serialized message but what is the data type ???
    msg: &[u8],
    aad: &[u8],
    pub_key: &<Kem as KemTrait>::PublicKey,) -> (<Kem as KemTrait>::EncappedKey, Vec<u8>, AeadTag<Aead>) {
         let mut csprng = StdRng::from_entropy();

         let (encapsulated_key, mut encryption_context) =
        hpke::setup_sender::<Aead, Kdf, Kem, _>(&OpModeS::Base, &pub_key, INFO_STR, &mut csprng)
        .expect("invalid server pubkey!");
        
        // seal in place will encrypt the plaintext in place if success
        let mut msg_copy = msg.to_vec();
        let tag = encryption_context.seal_in_place_detached(&mut msg_copy, aad).expect("encryption failed!");

// i honestly do not think this code is good because 
// https://github.com/rozbb/rust-hpke/blob/HEAD/examples/client_server.rs
// never uses msg variable in encyrpting
        // Rename for clarity
    let ciphertext = msg_copy;

        // return
        (encapsulated_key, ciphertext, tag)
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
    let aad = b"First encrypted message";

    // Marshall the message into bincode
    let serialized: Vec<u8> = bincode::serialize(&msg).unwrap();

    let (encapsulated_key, ciphertext, tag) = encrypt_msg(&serialized, aad, &pub_key);

    let encapsulated_key_bytes = encapsulated_key.to_bytes();
    let tag_bytes = tag.to_bytes();

    // Open a connection to the server
    match TcpStream::connect("127.0.0.1") {

        Ok(mut stream) => {
            println!("Connected to the server!");

            // send over the cyphertext

            //  Alice sends the encapsulated key, message ciphertext, AAD, and auth tag to Bob
            // how is bob supposed to nicely read it? should I create a struct for that? 

            stream.write(&ciphertext).unwrap();

            println!("Awaiting reply");

            // I DON"T REMEMBER WHY DID I PUT IT HERE ??
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
