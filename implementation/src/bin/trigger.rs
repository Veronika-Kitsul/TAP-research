use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::{TcpListener, Shutdown};
use implementation::format::{Message,MessageType,TransmissionData};
use bincode;
use std::thread;
use std::io::BufReader;
use std::fs::File;
use rand::prelude::*;
use hpke::{
    aead::{AeadTag, ChaCha20Poly1305},
    kdf::HkdfSha384,
    kem::X25519HkdfSha256,
    Kem as KemTrait, OpModeS, Serializable, Deserializable
};

// more resources on the types of them
type Kem = X25519HkdfSha256;
type Aead = ChaCha20Poly1305;
type Kdf = HkdfSha384;

// CHANGE IT DEPENDING ON THE SESSION ??????
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

        println!("encrypted data in the trigger");
        TransmissionData{encapped_key: encapsulated_key_vec, cyphertext: ciphertext}
    }

fn handle_tap(mut stream: TcpStream, pub_key: <Kem as KemTrait>::PublicKey) {
    let mut data = [0 as u8; 5000]; 
    while match stream.read(&mut data) {
        Ok(size) => {
            let aad = b"";
            let content = test1();

            let msg = Message {
                msgtype: MessageType::TriggerToTAP,
                contents: content.as_bytes().to_vec(),
            };

            let serialized: Vec<u8> = bincode::serialize(&msg).unwrap();

            // encrypt the message 
            let data = encrypt_msg(&serialized, aad, &pub_key);
            let data_serialized: Vec<u8> = bincode::serialize(&data).unwrap();
            stream.write(&data_serialized).unwrap();
            true
        },
        Err(e) => {
            println!("An error occured, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}
}

fn main() {
    // public key retrieval
    let f = File::open("public.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut pub_key_bytes = Vec::new();
    
    // Read file into vector.
    reader.read_to_end(&mut pub_key_bytes).unwrap();


    // Listen for connections on a loop
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // deserialize into private key type from the file
                let pub_key: <Kem as KemTrait>::PublicKey =
                    Deserializable::from_bytes(&pub_key_bytes).unwrap();    

                thread::spawn(move || {
                    println!("in the thread");
                    handle_tap(stream, pub_key);
                });

                println!("Terminated.");
            }

            Err(e) => {
                println!("Error: {}", e)
            }
        }
    }
    drop(listener);
}


fn test1() {
    
}