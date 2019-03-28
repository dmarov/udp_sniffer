use std::net::UdpSocket;
use std::{ env, thread, time };

fn main() -> std::io::Result<()> {


    let args: Vec<String> = env::args()
        .collect();

    let mut bind_to = String::from("127.0.0.1:8081");

    for i in 0..args.len() {

        if String::from("--bind-to") == args[i] {
            bind_to = (args[i + 1]).clone();
        }
    }

    let mut buf = [0; 10];

    let socket = UdpSocket::bind(bind_to)
        .unwrap();

    let sleep_duration = time::Duration::from_millis(1000);

    loop {

        socket.recv(&mut buf)
            .unwrap();

        println!("received {:?}", buf);
        thread::sleep(sleep_duration);
    }

    Ok(())
}
