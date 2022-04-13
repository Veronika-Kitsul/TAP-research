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

type Kem = X25519HkdfSha256;
type Aead = ChaCha20Poly1305;
type Kdf = HkdfSha384;

// but do i get this info from the client as well???
const INFO_STR: &[u8] = b"example session";

fn decrypt_msg(
    ciphertext: &[u8],
    priv_key: &<Kem as KemTrait>::PrivateKey,
    encapped_key: &EncappedKey<Kem::Kex>,
    info: &[u8]
) -> Vec<u8>
{


    // Now let the server decrypt the message. The to_bytes() calls returned a GenericArray, so we
    // have to convert them to slices before sending them
    let decrypted_msg = server_decrypt_msg(
        server_privkey_bytes.as_slice(),
        encapped_key_bytes.as_slice(),
        &ciphertext,
        associated_data,
        tag_bytes.as_slice(),
    );

    // Initiates a decryption context given a private key sk_recip and an encapsulated key 
    // which was encapsulated to sk_recip's corresponding public key
    let mut decryption_context =
    hpke::setup_receiver::<Aead, Kdf, Kem>(
        &OpModeR::Base,
        &priv_key,
        &encapsulated_key,
        INFO_STR,
    ).expect("failed to set up receiver!");
// To open without allocating:
//     decryption_context.open_in_place_detached(&mut ciphertext, aad, &auth_tag)
// To open with allocating:
    let plaintext = decryption_context.open(&ciphertext, aad).expect("invalid ciphertext!");
    println!("{}", plaintext);
    plaintext
}

fn handle_client(mut stream: TcpStream) {

    // I think I had to use this cap on the amount of symbols because it wants a speicified type but how do I not cap it?
    let mut data = [0 as u8; 2000]; 
    while match stream.read(&mut data) {
        Ok(size) => {

            // decrypt the message 
            decrypt_msg(data, );
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
        let f = File::open("private.txt").unwrap();
        let mut reader = BufReader::new(f);
        let mut priv_key_bytes = Vec::new();
        
        // Read file into vector.
        reader.read_to_end(&mut priv_key_bytes).unwrap();

        // deserialize into private key type from the file
        let priv_key: <Kem as KemTrait>::PrivateKey =
            Deserializable::from_bytes(&priv_key_bytes).unwrap();

    let listener = TcpListener::bind("127.0.0.1").unwrap();

    // Listen for connections on a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    
                    handle_client(stream)

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
