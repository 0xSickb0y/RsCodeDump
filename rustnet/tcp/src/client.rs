use std::process;
use std::net::TcpStream;
use std::io::{self, BufRead, BufReader, Write};

pub fn run(srv_addr: &str, srv_port: &str) {
    println!("rustnet TCP client");
    let full_addr = format!("{}:{}", srv_addr, srv_port);

    let mut stream = TcpStream::connect(&full_addr)
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect to {}: {}", full_addr, e);
            process::exit(1);
    });

    println!("Connected to server at: {}", full_addr);

    // Clone the stream for reading
    let mut reader = BufReader::new(stream.try_clone()
        .expect("Failed to clone stream")
    );

    loop {
        // Prepare to read from stdin
        io::stdout().flush().expect("Failed to flush stdout");

        // Handle stdin reading
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .unwrap_or_else(|e| {
                eprintln!("Failed to read from stdin: {}", e);
                process::exit(1);
            });
        
        // Send message to server
        stream.write_all(input.as_bytes())
            .unwrap_or_else(|e| {
                eprintln!("Failed to send message to server: {}", e);
                process::exit(1);
            });

        // Prepare to read server response
        let mut response = String::new();
        reader.read_line(&mut response)
            .unwrap_or_else(|e| {
                eprintln!("Failed to read server response: {}", e);
                process::exit(1);
            });

        // Output server response
        print!("[{}]: {}", full_addr, response)
    }
}
