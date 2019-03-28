use std::net::UdpSocket;
use std::{ env, thread, time };

fn main() -> std::io::Result<()> {


    let args: Vec<String> = env::args()
        .collect();

    let mut bind_to = String::from("0.0.0.0:8080");
    let mut send_to = String::from("");

    for i in 0..args.len() {

        if String::from("--bind-to") == args[i] {
            bind_to = (args[i + 1]).clone();
        } else if String::from("--send-to") == args[i] {
            send_to = (args[i + 1]).clone();
        }
    }

    let mut buf = [0; 10];
    buf[1] = 1;
    buf[2] = 2;
    buf[3] = 3;
    buf[4] = 4;

    let socket = UdpSocket::bind(bind_to)
        .unwrap();

    let sleep_duration = time::Duration::from_millis(1000);

    loop {

        socket.send_to(&buf, &send_to)
            .unwrap();

        println!("sent");
        thread::sleep(sleep_duration);
    }

    Ok(())
}
