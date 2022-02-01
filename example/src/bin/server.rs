use std::net::TcpListener;
use std::thread;

fn main() {

    let listener = TcpListener::bind("").unwrap();

    // Listen for connections on a loop
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(move|| {
            // decrypt a message with a key

            // When it gets connection, decrypt message

            // Unmarshall back to message struct

            // Do something with the message
        });
    }

    drop(listener);
}
