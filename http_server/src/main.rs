use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use http_server::ThreadPool;

fn main() {
    let addr = "127.0.0.1:7878";
    let listner = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(4);
    println!("start listening on {}", addr);

    // `.take(2)` for debug.
    for stream in listner.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream))
    }

    println!("Shutting down.");
}

#[allow(clippy::unused_io_amount)]
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // for debug
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, file_path) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 Not found\r\n\r\n", "404.html")
    };

    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
