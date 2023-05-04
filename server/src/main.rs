use std::io;
use std ::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};

fn main() -> io::Result<()> {
    const CONNECTION: &str = "127.0.0.1:4695";
    let receiver_listener = TcpListener::bind(CONNECTION).expect("Failed to connect with sender");
    
    let clients = Arc::new(Mutex::new(Vec::new()));

    for stream in receiver_listener.incoming() {
        let mut stream = stream.unwrap();

        let client_address = stream.peer_addr().expect("Failed to get client address");
        println!("Accepted connection from: {}", client_address);

        let clients_clone = clients.clone();
        clients_clone.lock().unwrap().push(stream.try_clone().unwrap());

        let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
        let handle = thread::spawn(move || {
            handle_sender(&mut stream, &clients_clone).expect("Failed to write message");
        });
        thread_vec.push(handle);
    }

    Ok(())
}

fn handle_sender(stream: &mut TcpStream, clients: &Arc<Mutex<Vec<TcpStream>>>) -> io::Result<()> {
    // handle multiple clients 
    let mut buf = [0; 512];
    loop {
        // Read incoming messages
        match stream.read(&mut buf) {
            Ok(0) => break, // Client disconnected
            Ok(msg) => {
                let message = &buf[..msg];
                // Send the message to all connected clients
                let mut clients = clients.lock().unwrap();
                for client in clients.iter_mut() {
                    if client.peer_addr().unwrap() == stream.peer_addr().unwrap() {
                        continue;
                    }
                    client.write_all(message).unwrap();
                }
            }
            Err(_) => break, // Client disconnected or there was an error
        }
    }

    Ok(())
}