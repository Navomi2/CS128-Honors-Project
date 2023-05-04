use std::io;
use std::time;
use std ::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() -> io::Result<()> {
    const CONNECTION: &str = "127.0.0.1:4695";
    let receiver_listener = TcpListener::bind(CONNECTION).expect("Failed to connect with sender");
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    for stream in receiver_listener.incoming() {
        let stream = stream.expect("failure");
        let handle = thread::spawn(move || {
            handle_sender(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
        });
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}

fn handle_sender(mut stream: TcpStream) -> io::Result<()> {
    // handle multiple access stream
    let mut buf = [0;512];
    for _ in 0..1000 {
        // receiver gets a message from sender
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}