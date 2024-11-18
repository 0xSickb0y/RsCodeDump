use std::net::TcpStream;
use std::{str, env, process};
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect(); // Collects the command-line arguments into a vector of strings.
    
    if args.len() != 3 {
        println!("Usage: ./client <srv_addr> <srv_port>");
        process::exit(1);
    }
    
    println!("rustnet TCP client");

    let srv_addr = &args[1];
    let srv_port = &args[2];
    let full_addr = format!("{}:{}", srv_addr, srv_port);

    let mut stream = TcpStream::connect(&full_addr)
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect to {}: {}", full_addr, e);
            process::exit(1);
        });

    println!("Connected to server at: {}", full_addr);

    loop {
        let mut input = String::new();
        let mut buf: Vec<u8> = Vec::new();
        
        // Handle stdin reading
        io::stdin().read_line(&mut input)
            .unwrap_or_else(|e| {
                eprintln!("Failed to read from stdin: {}", e);
                process::exit(1);
            });
        

        // Send message to server
        stream.write(input.as_bytes())
            .unwrap_or_else(|e| {
                eprintln!("Failed to send message to server: {}", e);
                process::exit(1);
            });

        // Read server response
        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buf)
            .unwrap_or_else(|e| {
                eprintln!("Could not read into buffer: {}", e);
                process::exit(1);
            });

        // Output server response
        println!("{}", str::from_utf8(&buf)
            .unwrap_or_else(|e| {
                eprintln!("Could not write buffer as string: {}", e);
                process::exit(1);
            })
        );
    }
}
