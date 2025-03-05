use std::net::TcpListener;

use hello_world::{handle_connection::handle_connection, threadpool};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = threadpool::ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.")
}