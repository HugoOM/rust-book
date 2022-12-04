use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::{thread, time};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 4096];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // let response = r#"HTTP/1.1 200 OK

    //     <html>
    //         <body>
    //             <h1>Hi!</h1>
    //         </body>
    //     </html>
    // "#;
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let response = r#"HTTP/1.1 404 NOT FOUND

            <html>
                <body>
                    <h1>Page Not Found</h1>
                </body>
            </html>
        "#;

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    thread::sleep(time::Duration::from_secs(5));
}
