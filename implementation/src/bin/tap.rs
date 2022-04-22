use std::net::{TcpStream, Shutdown};
use implementation::format::{TransmissionData};
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
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
    let requested_data = b"Most recent email.";
    
    // connect to the trigger on port 8080
    match TcpStream::connect("127.0.0.1:8080") {

        Ok(mut stream) => {
            println!("Connected to the trigger!");
            println!("Poll time: {:?}", SystemTime::now());
            stream.write(requested_data).unwrap();
            println!("wrote the data");
            let mut received_data = [0 as u8; 5000];

            while match stream.read(&mut received_data) {
                Ok(size) => {
                    // connect to the action service
                    match TcpStream::connect("127.0.0.1:8082") {

                        Ok(mut stream) => {
                            println!("Connected to the action!");
                            stream.write(&received_data[0..size]).unwrap();
                        },
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                    }
                    false
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                    false
                }
            }{}
        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
    
}
