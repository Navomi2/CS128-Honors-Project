use std::io::{stdin, Read, Write};
use std::net::TcpStream;
use std::thread;
use std::io;

fn main() {
    // port number
    const PORT: &str = "127.0.0.1:4695";
    // wasn't sure what an average message size is, this number can be changed later
    const MSG_SIZE: usize = 1000;

    // client connects to port 
    let mut stream = TcpStream::connect(PORT).expect("Failed to connect to port");
    stream.set_nonblocking(true).expect("Failed to set nonblocking");

    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
    
    // thread for recieving messages from server
    thread::spawn(move || loop {
        // reading messages from the TCP stream
        let mut msg = vec![0; MSG_SIZE];
        match stream.read(&mut msg) {
            Ok(_) => {
                let msg_bytes: Vec<u8> = msg.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg_str = String::from_utf8(msg_bytes).expect("Invalid utf8 message");
                println!("Received message from server: {:?}", msg_str);
            }
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server was severed");
                break;
            }
        }
    });


    // while main thread continues accepting input
    // letting background thread handle sending it to server
    println!("Write your message: ");
    loop {
        
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Could not read input");
        let input = input.trim().as_bytes().to_vec();
        
        // trying to send recent input to reciever
        stream_clone.write_all(&input).expect("Writing to TCP stream failed");

        if input.len() == 0 {
            break;
        }
    }
}