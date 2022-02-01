use std::net::TcpListener;

fn main() {

    
    let listener = TcpListener::bind("").unwrap();
//do we need a thread pool?
    //let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // pool.execute(|| {
        //     handle_connection(stream);
        // });
    }
    // Listen for connections on a loop

    // When it gets connection, decrypt message

    // Unmarshall back to message struct

    // Do something with the message
}
