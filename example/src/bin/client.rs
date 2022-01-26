use example::format::{Message,MessageType};

fn main() {
    // Create a message
    let msg = Message {
        msgtype: MessageType::Type1,
        contents: b"Hello".to_vec(),
    };

    // Marshall the message into byte string
    // Using the serde crate, probably marshal into bincode

    // Encrypt the message into a ciphertext
    // Use public key encryption

    // Open a connection to the server

    // Send ciphertext to the server

    // Gets a response from server
}
