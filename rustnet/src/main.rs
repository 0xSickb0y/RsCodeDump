use std::{env, process};

mod tcp_client;
mod tcp_server;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  cargo run tcp_client <server-address> <port>");
        eprintln!("  cargo run tcp_server <bind-address> <port>");
        process::exit(1);
    }

    match args[1].as_str() {
        "tcp_client" => {
            if args.len() != 4 {
                eprintln!("tcp_client usage: cargo run tcp_client <server-address> <port>");
                process::exit(1);
            }
            tcp_client::run(&args[2], &args[3]);
        },
        "tcp_server" => {
            if args.len() != 4 {
                eprintln!("tcp_server usage: cargo run tcp_server <bind-address> <port>");
                process::exit(1);
            }
            
            // Handle different bind address options
            let bind_addr = match args[2].as_str() {
                "127.0.0.1" => "127.0.0.1",
                "any" => "0.0.0.0", // Listen on all network interfaces
                _ => {
                    eprintln!("Invalid bind address. Use 127.0.0.1 or any.");
                    process::exit(1);
                }
            };
            
            tcp_server::run(bind_addr, &args[3]);
        },
        _ => {
            eprintln!("Invalid command. Use 'tcp_client' or 'tcp_server'.");
            process::exit(1);
        }
    }
}
