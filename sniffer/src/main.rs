extern crate pnet;
extern crate serde_json;

use pcap::{ Capture, Device };
use std::env;
use pnet::packet::{ Packet, udp::UdpPacket, ethernet::EthernetPacket, ipv4::Ipv4Packet };
use chrono::{ Local };

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

    loop {
        while let Ok(packet) = cap.next() {

            let vec = packet.to_vec();
            let slice = vec.as_slice();
            let ethernet_pdu = EthernetPacket::new(slice).unwrap();
            let ethernet_payload = ethernet_pdu.payload();
            let ipv4_pdu = Ipv4Packet::new(ethernet_payload).unwrap();
            let ipv4_payload = ipv4_pdu.payload();
            let udp_pdu = UdpPacket::new(ipv4_payload).unwrap();
            let udp_payload = udp_pdu.payload();
            let date = Local::now();
            let time = date.format("%Y:%m:%d %H:%M:%S").to_string();

            let json = serde_json::json!({
                    "date": time,
                    "source_mac": ethernet_pdu.get_source().to_string(),
                    "destination_mac": ethernet_pdu.get_destination().to_string(),
                    "source_ip": ipv4_pdu.get_source(),
                    "destination_ip": ipv4_pdu.get_destination(),
                    "source_port": udp_pdu.get_source(),
                    "destination_port": udp_pdu.get_destination(),
                    "payload": udp_payload,
            }).to_string();

            println!("{:?};", json);
            // println!("sniffed ethernet {:?}", ethernet_payload);
            // println!("sniffed udp {:?}", udp_payload);
            // println!("sniffed {:?}", content);
        }
    }

    Ok(())
}
