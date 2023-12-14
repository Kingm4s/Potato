use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use rustyline::Editor;

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 512];
    let client_address = stream.peer_addr().expect("Failed to get client address");

    println!("New connection from: {}", client_address);

    {
        let mut clients = clients.lock().expect("Failed to lock clients");
        clients.push(stream.try_clone().expect("Failed to clone stream"));
    }

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected", client_address);
                break;
            }
            Ok(_) => {
                let received_message = String::from_utf8_lossy(&buffer[..]);
                println!("Received message from {}: {}", client_address, received_message);

                // Broadcasting the message to all clients
                let clients = clients.lock().expect("Failed to lock clients");
                for mut client in &*clients {
                    if let Err(_) = client.write_all(&buffer) {
                        println!("Error writing to client");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", client_address, e);
                break;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081")?;

    println!("Server listening on 127.0.0.1:8080");

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients_clone = Arc::clone(&clients);
                thread::spawn(move || {
                    handle_client(stream, clients_clone);
                });
            }

        }
    }

    Ok(())
}
