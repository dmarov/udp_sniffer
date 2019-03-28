extern crate pnet;

use pcap::{ Capture, Device };

use std::env;
use pnet::packet::{ Packet, udp::UdpPacket };

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args()
        .collect();

    let mut filter = String::from("");
    let mut device = String::from("lo0");

    for i in 0..args.len() {

        if String::from("--list") == args[i] {

            let devices = Device::list()
                .unwrap();
            println!("{:?}", devices);
            std::process::exit(0);
        }

        if String::from("--device") == args[i] {
            device = (args[i + 1]).clone();
        }

        if String::from("--filter") == args[i] {
            filter = (args[i + 1]).clone();
        }
    }

    let mut cap = Capture::from_device(device.as_str())
        .unwrap()
        .buffer_size(1000)
        .open()
        .unwrap();

    cap.filter(filter.as_str())
        .unwrap();

    while let Ok(packet) = cap.next() {

        let vec = packet.to_vec();
        let slice = vec.as_slice();
        let udp_pdu = UdpPacket::new(slice).unwrap();
        let payload = udp_pdu.payload();
        let content = &payload[24..];

        println!("sniffed {:?}", content);
    }

    Ok(())
}
