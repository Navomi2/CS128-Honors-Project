use std::io::{stdin, ErrorKind, Read, Write};
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

fn main() {
    //port number
    const PORT: &str = "127.0.0.1:4695";
    //wasn't sure what an average message size is, this number can be changed later
    const MSG_SIZE: usize = 150;

    //client connects to port 
    let mut client = TcpStream::connect(PORT).expect("Failed to connect to port");

    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

    //thread is reading data from TCP stream and sending it to the server
    thread::spawn(move || loop {
        //reading data from the TCP stream
        let mut vect = vec![0; MSG_SIZE];
        let sender = tx.clone();
        match client.read(&mut vect) {
            Ok(_) => {
                let msg_bytes: Vec<u8> = vect.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();           
                sender.send(msg_bytes);
            }
            Err(_) => break,
        }

        //if there is a message it is written to the server
        match rx.try_recv() {
            Ok(msg) => {
                let vect = msg;
                client.write_all(&vect).expect("Writting failed");
            }
            Err(_) => break,
        }
    });
    
    //while main thread continues accepting input (not a loop rn but probably should be one to continuously take in messages)
    //letting background thread handle sending it to server
    println!("Write your message: ");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Could not read input");
    
    //trying to send recent input to reciever
    tx.send(input.clone().into_bytes());
}