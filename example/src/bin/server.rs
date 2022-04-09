use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Write};
use rand::prelude::*;
use hpke::{
    aead::{AeadTag, ChaCha20Poly1305},
    kdf::HkdfSha384,
    kem::X25519HkdfSha256,
    Kem as KemTrait, OpModeS, Serializable, Deserializable
};

fn decrypt_msg(
    server_sk_bytes: &[u8],
    encapped_key_bytes: &[u8],
    ciphertext: &[u8],
    aad: &[u8],
    tag_bytes: &[u8],
) -> Vec<u8>
{
    let tag = AeadTag::<Aead>::from_bytes(tag_bytes).expect("could not deserialize AEAD tag");

    // decapsulate and derive the shared secret to create AEAD context
    let mut receiver_ctx = hpke::setup_receiver::<Aead, Kdf, Kem>
                (&OpModeR::Base,INFO_STR).expect("failed to set up receiver!");

    let mut plaintext = ciphertext.to_vec();
    receiver_ctx.open_in_place_detached(&mut plaintext, aad, &tag).expect("invalid ciphertext");

    plaintext
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 2000]; 
    while match stream.read(&mut data) {
        Ok(size) => {
            decrypt_msg(data);
            true
        }
        Err(e) => {
            println!("An error occured, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
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
        let pub_key: <Kem as KemTrait>::PrivateKey =
            Deserializable::from_bytes(&pub_key_bytes).unwrap();

    let listener = TcpListener::bind("127.0.0.1").unwrap();

    //pass key

    // Listen for connections on a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    
                    handle_client(stream)

                // decrypt a message with a key

                // When it gets connection, decrypt message

                // Unmarshall back to message struct

                // Do something with the message
                    
                });
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
        drop(listener);
    }
}
