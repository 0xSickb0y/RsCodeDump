use std::io::{Read, Error};
use std::{env, thread, process};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Received connection: {}", stream.peer_addr()?); // Returns the socket address of the remote peer of this TCP connection.
    let mut buf = [0;512];
    
    loop {
        let bytes_read = stream.read(&mut buf)?; // Read incoming data into the buffer
        if bytes_read == 0 {
            println!("Connection closed: {}", stream.peer_addr()?);
            return Ok(()); // if no bytes are read, disconnect the client and return `Ok()` to signal successful handling
        }

        println!("[{}] {}", stream.peer_addr()?, // Prints the client address with the message
            String::from_utf8_lossy(&buf[..bytes_read])) // Converts a slice of bytes to a string, including invalid characters.
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ./server <srv_addr> <srv_port>");
        process::exit(1);
    }

    let srv_addr = &args[1];
    let srv_port = &args[2];
    let full_addr = format!("{}:{}", srv_addr, srv_port);

    
    let listener = TcpListener::bind(&full_addr) // TCP socket listening on localhost.
    .unwrap_or_else(|e| {
        println!("Could not bind to address: {}: {}", full_addr, e);
        process::exit(1)
    });
    
    println!("rustnet TCP server");

    for stream in listener.incoming() { // Returns an iterator over the connections being received on the listener.
        match stream {
            Ok(stream) => { // In case of success, spawn a new thread for the client.
                thread::spawn(move || { // `move` keyword ensures that the stream is moved into the closure, meaning the thread gets ownership of it and can safely use it in the new thread.
                    handle_client(stream)
                        .unwrap_or_else( // .unwrap_or_else handles the error case without panicking
                            |error| // Closure parameter. The closure will accept an argument named `error` when called.
                            eprintln!("{:?}", error) // Closure body. What the closure does when it’s executed. (Prints to stderr using debug formatting)
                        )                 
                });
            }
            Err(e) => { // In case of error, print line to stderr, and move on to the next connected client.
                eprintln!("Connection failed: {}", e)
            }
        }
    }
}
