use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use rustyline::Editor;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).expect("Failed to read from socket");
    let received_message = String::from_utf8_lossy(&buffer[..]);
    println!("Received message: {}", received_message);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut rl = Editor::<()>::new();
                if let Ok(line) = rl.readline("Press Enter to send a message: ") {
                    println!("Sending message: {}", line);
                    stream.write(line.as_bytes()).expect("Failed to write to socket");
                }

                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}


