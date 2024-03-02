use clap::Parser;
use std::io::prelude::*;
use std::net::TcpStream;

/// Remote Control for Nikis Quadcopter
#[derive(Parser)]
struct Cli {
    /// The IP address of the quadcopter
    ip: String,
    port: u16,
}

fn main() {
    let args = Cli::parse();
    println!("Remote Control for Nikis Quadcopter starts with {}:{}", args.ip, args.port);

    if let Ok(stream) = TcpStream::connect(format!("{}:{}", args.ip, args.port)) {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    };



}
