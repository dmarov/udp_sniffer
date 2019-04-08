extern crate pnet;
extern crate serde_json;
extern crate hex;

use pcap::{ Capture, Device };
use std::{ env, fs::File };
use pnet::packet::{ Packet, udp::UdpPacket, ethernet::EthernetPacket, ipv4::Ipv4Packet };
use chrono::{ Local };
use std::io::prelude::*;
use hex::ToHex;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args()
        .collect();

    let mut filter = String::from("");
    let mut device = String::from("lo0");
    let mut write_to = String::from("data.log");

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

        if String::from("--write-to") == args[i] {
            write_to = (args[i + 1]).clone();
        }
    }

    let mut cap = Capture::from_device(device.as_str())
        .unwrap()
        .buffer_size(1000)
        .open()
        .unwrap();

    cap.filter(filter.as_str())
        .unwrap();
    let mut file = File::create(write_to)?;

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
            let mut hex_udp_payload = String::new();

            for byte in udp_payload.iter() {

                hex_udp_payload.push_str((format!("{:02X}", byte)).as_str().chars().rev().collect::<String>().as_str());
                hex_udp_payload.push_str("|");
            }

            // udp_payload.write_hex_upper(&mut hex_udp_payload).unwrap();

            let mut json = serde_json::json!({
                    "date": time,
                    "payload": hex_udp_payload,
                    "len": udp_payload.len(),
            }).to_string();

            println!("{:?}", json);
            json.push_str("\r\n");
            file.write_all(json.as_bytes())?;
        }
    }

    Ok(())
}
