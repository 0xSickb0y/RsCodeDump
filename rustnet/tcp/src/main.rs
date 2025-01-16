use std::env;
use std::process;

mod client;
mod server;

fn print_help_exit() {
    eprintln!("Usage:");

    eprintln!("  ./tcp server <bind-address> <port>");
    eprintln!("  ./tcp client <server-address> <port>");
    eprintln!();
    eprintln!("  cargo run server <bind-address> <port>");
    eprintln!("  cargo run client <server-address> <port>");
    
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        print_help_exit();
    }

    match args[1].as_str() {
        "client" => {
            client::run(&args[2], &args[3]);
        },
        "server" => {
            server::run(&args[2], &args[3]);
        },
        other => {
            eprintln!("Invalid option: {other}");
            print_help_exit();
        }
    }
}
