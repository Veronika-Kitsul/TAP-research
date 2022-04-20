use std::net::{TcpListener, TcpStream, Shutdown};
use std::fs::File;
use implementation::format::{TransmissionData};
use std::io::{Read, Write};
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
            stream.write(&requested_data).unwrap();

            let mut received_data = [0 as u8; 5000];

            match stream.read_exact(&mut received_data) {
                Ok(data) => {

                    println!("{:?}", received_data);

                    // connect to the action service
                    match TcpStream::connect("127.0.0.1:8082") {

                        Ok(mut stream) => {
                            println!("Connected to the trigger!");
                            stream.write(&received_data).unwrap();
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
    }
}
