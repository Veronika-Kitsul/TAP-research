use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1").unwrap();

    //pass key

    // Listen for connections on a loop
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // how many threads -- 4
        thread::spawn(move|| {
            // decrypt a message with a key

            // When it gets connection, decrypt message

            // Unmarshall back to message struct

            // Do something with the message
        });
    }

    drop(listener);
}
