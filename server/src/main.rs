use std::net::UdpSocket;
use std::env;

fn main() -> std::io::Result<()> {


    let args: Vec<String> = env::args().collect();
    let mut buf = [0; 10000];
    let mut socket = UdpSocket::bind("0.0.0.0:8080");

    println!("Hello, world!");
    Ok(())
}
