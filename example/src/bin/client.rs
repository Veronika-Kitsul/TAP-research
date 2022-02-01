extern crate openssl;

use std::io::Write;
use std::net::TcpStream;
use example::format::{Message,MessageType};
use serde::{Serialize, Deserialize};
use bincode;
use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;

fn main() {
    // Create a message
    let msg = Message {
        msgtype: MessageType::Type1,
        contents: b"Hello".to_vec(),
    };

    // Marshall the message into byte string
    // Using the serde crate, probably marshal into bincode
    let serialized = bincode::serialize(&msg).unwrap();


    // Encrypt the message into a ciphertext
    // Use public key encryption
    let encrypted = b"Hello";
  
    // Open a connection to the server
    match TcpStream::connect("") {

        Ok(mut stream) => {
            println!("Connected to the server!");

            // Send ciphertext to the server
            stream.write(encrypted).unwrap();
        }
        
        Err(e) => {
            println!("There was an error.");
        }
    }

    // Gets a response from server
    // how do i receive a message
}
