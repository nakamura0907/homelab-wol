use std::{error::Error, net::UdpSocket};

mod cli;

use crate::cli::Options;

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::new();

    run(options)?;

    Ok(())
}

/// メインエントリポイントの実行
fn run(options: Options) -> Result<(), Box<dyn Error>> {
    println!("MACアドレス = {}", options.mac_address);

    send_magic_packet(options.mac_address)?;

    Ok(())
}

fn send_magic_packet(mac_address: String) -> Result<(), Box<dyn Error>> {
    let mac_bytes = parse_mac_address(&mac_address);
    let packet = create_magic_packet(&mac_bytes);

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    socket.send_to(&packet, "255.255.255.255:9")?;

    Ok(())
}

fn create_magic_packet(mac_bytes: &[u8; 6]) -> [u8; 102] {
    let mut packet = [0xFFu8; 102];

    for i in 0..16 {
        packet[6 + i * 6..6 + (i + 1) * 6].copy_from_slice(mac_bytes);
    }

    packet
}

fn parse_mac_address(mac_address: &str) -> [u8; 6] {
    let bytes: Vec<u8> = mac_address
        .split(':')
        .map(|s| u8::from_str_radix(s, 16).unwrap_or(0))
        .collect();
    bytes.try_into().unwrap_or([0; 6])
}
