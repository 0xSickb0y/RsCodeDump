use std::{thread, process};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error, stdin, stdout};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Received connection: {}", stream.peer_addr()?); // Returns the socket address of the remote peer of this TCP connection.
    let mut buf = [0; 1024];
    let stdin = stdin();
    let mut input = String::new();
    
    loop {
        let bytes_read =  match stream.read(&mut buf) { // Read incoming data into the buffer 
            Ok(0) => {
                println!("Connection closed: {}", stream.peer_addr()?);
                return Ok(());
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return Err(e);
            }
        };

        // Print received message
        println!("[{}] {}", stream.peer_addr()?,
            String::from_utf8_lossy(&buf[..bytes_read])
        );

        // Prompt for server input
        print!(">>> ");
        stdout().flush()?;
        input.clear();
        stdin.read_line(&mut input)?;

        // Send server's message to client
        stream.write_all(input.as_bytes())?;
    }
}

pub fn run(bind_addr: &str, bind_port: &str) {
    let full_addr = format!("{}:{}", bind_addr, bind_port);
    let listener = TcpListener::bind(&full_addr) // TCP socket listening on localhost.
        .unwrap_or_else(|e| {
            println!("Could not bind to address: {}: {}", full_addr, e);
            process::exit(1)
    });
    
    println!("rustnet TCP server listening on {}", full_addr);

    for stream in listener.incoming() { // Returns an iterator over the connections being received on the listener.
        match stream {
            Ok(stream) => { // In case of success, spawn a new thread for the client.
                thread::spawn(move || { // `move` keyword ensures that the stream is moved into the closure, meaning the thread gets ownership of it and can safely use it in the new thread.
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Failed to handle client: {:?}", e);
                    }
                });               
            }
            Err(e) => { // In case of error, print line to stderr, and move on to the next connected client.
                eprintln!("Connection failed: {}", e)
            }
        }
    }
}
