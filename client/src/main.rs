use std::io::{stdin, Read, Write};
use std::net::{TcpListener,TcpStream};
//use std::thread;
use std::io;

fn main() {
    //port number
    const PORT: &str = "127.0.0.1:4695";
    //wasn't sure what an average message size is, this number can be changed later
    const MSG_SIZE: usize = 1000;

    //client connects to port 
    let mut stream = TcpStream::connect(PORT).expect("Failed to connect to port");
    stream.set_nonblocking(true).expect("Failed to set nonblocking");

    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
    // read messages from the terminal and send them to the server
    loop {
        // check if there is any message from the user and send it to the server
        let mut input = String::new();
        println!("Enter your message: ");
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().as_bytes().to_vec();
                // Send the message to the server
                stream.write_all(&input).expect("Failed to send");
            }
            Err(_) => (),
        }

        // reading messages from the TCP stream
        let mut input = vec![0; MSG_SIZE];
        match stream.read(&mut input) {
            Ok(_) => {
                let msg_bytes: Vec<u8> = input.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                tx.send(msg_bytes).expect("Failed to send");
            }
            Err(_) => (),
        }

        // check if there is any message from the server and write it to the terminal
        match rx.try_recv() {
            Ok(msg) => {
                println!("Received message from server: {:?}", msg);
            }
            Err(_) => (),
        }
    }
}
/* 
let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
let sender = tx.clone();
//thread is reading data from TCP stream and sending it to the server
thread::spawn(move || loop {
    //if there is a message it is written to the server
    match rx.try_recv() {
        Ok(msg) => {
            let input = msg;
            println!("Sending message to server: {:?}", input);
            stream_clone.write_all(&input).expect("Writting failed");
        }
        Err(_) => break,
    } 

    //reading messages from the TCP stream
    let mut input = vec![0; MSG_SIZE];
    match stream_clone.read(&mut input) {
        Ok(_) => {
            let msg_bytes: Vec<u8> = input.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();           
            sender.send(msg_bytes).expect("Failed to send");
            println!("Received message from server: {:?}", input);
        }
        Err(_) => break,
    }       
});

//while main thread continues accepting input 
//letting background thread handle sending it to server
println!("Write your message: ");
loop {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Could not read input");
    let input = input.trim().as_bytes().to_vec();
    
    //trying to send recent input to reciever
    stream.write_all(&input).expect("Writing to TCP stream failed");

    if input.len() == 0 {
        break;
    }
}
*/