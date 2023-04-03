//use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    const PORT: &str = "127.0.0.1:4695";
    
    //server listening for connections to port
    let server = TcpListener::bind(PORT).expect("Listener failed to bind");

}