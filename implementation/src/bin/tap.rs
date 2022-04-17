use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::fs::File;
use implementation::format::{Message,MessageType,TransmissionData};
use std::io::BufReader;
use std::io::{Read, Write};
use rand::prelude::*;
use hpke::{
    aead::{AeadTag, ChaCha20Poly1305},
    kdf::HkdfSha384,
    kem::X25519HkdfSha256,
    Kem as KemTrait, OpModeR, Serializable, Deserializable
};

type Kem = X25519HkdfSha256;
type Aead = ChaCha20Poly1305;
type Kdf = HkdfSha384;

// but do i get this info from the client as well???
const INFO_STR: &[u8] = b"session";

fn handle_client(mut stream: TcpStream, priv_key: <Kem as KemTrait>::PrivateKey) {
    let mut data = [0 as u8; 5000]; 
    while match stream.read(&mut data) {
        Ok(size) => {

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
    // private key retrieval
        // set up the server 
        let f = File::open("private.txt").unwrap();
        let mut reader = BufReader::new(f);
        let mut priv_key_bytes = Vec::new();
        
        // Read file into vector.
        reader.read_to_end(&mut priv_key_bytes).unwrap();

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Listen for connections on a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // deserialize into private key type from the file
                let priv_key: <Kem as KemTrait>::PrivateKey =
                    Deserializable::from_bytes(&priv_key_bytes).unwrap();                
                thread::spawn(move || {
                    // how to give each thread its own priv key ????
                    handle_client(stream, priv_key)
                });
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }
        drop(&listener);
    }
}
