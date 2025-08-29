use std::net::UdpSocket;

use crate::errors::{MacAddressError, NetworkError, WolError};

/// Sends a Wake-on-LAN (WOL) magic packet.
pub fn send_magic_packet(mac_address: &str) -> Result<(), WolError> {
    let mac_bytes = parse_mac_address(mac_address)?;
    let packet = create_magic_packet(&mac_bytes);

    // Create a UDP socket bound to any available local address and port
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(NetworkError::SocketCreate)?;
    // Enable broadcast on the socket so it can send packets to the broadcast address
    socket
        .set_broadcast(true)
        .map_err(NetworkError::BroadcastSet)?;

    // Send the magic packet over UDP to the target broadcast address and port
    socket
        .send_to(&packet, "255.255.255.255:9")
        .map_err(NetworkError::Send)?;

    Ok(())
}

/// Creates a WOL magic packet from a 6-byte MAC address.
fn create_magic_packet(mac_bytes: &[u8; 6]) -> [u8; 102] {
    // Initialize the entire packet with 0xFF.
    let mut packet = [0xFFu8; 102];

    // Repeat the MAC address 16 times after the 6-byte header of 0xFF
    for i in 0..16 {
        packet[6 + i * 6..6 + (i + 1) * 6].copy_from_slice(mac_bytes);
    }

    packet
}

/// Parses a MAC address string into a 6-byte array.
fn parse_mac_address(mac_address: &str) -> Result<[u8; 6], MacAddressError> {
    // Split the MAC address string by ':' into 6 parts
    let parts: Vec<&str> = mac_address.split(':').collect();
    if parts.len() != 6 {
        return Err(MacAddressError::InvalidLength(parts.len()));
    }

    // Convert each hex string part into a u8 number
    let mut bytes = [0u8; 6];
    for (i, part) in parts.iter().enumerate() {
        bytes[i] = u8::from_str_radix(part, 16)
            .map_err(|_| MacAddressError::InvalidHex(part.to_string()))?;
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mac_address_valid() {
        let mac = "00:11:22:33:44:55";

        let bytes = parse_mac_address(mac).unwrap();

        assert_eq!(bytes, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    }

    #[test]
    fn test_parse_mac_address_errors() {
        struct TestCase<'a> {
            input: &'a str,
            expected: MacAddressError,
        }

        let cases = vec![
            TestCase {
                input: "00:11:22",
                expected: MacAddressError::InvalidLength(3),
            },
            TestCase {
                input: "00:11:22:33:44:55:66",
                expected: MacAddressError::InvalidLength(7),
            },
            TestCase {
                input: "00:11:22:33:44:zz",
                expected: MacAddressError::InvalidHex("zz".to_string()),
            },
            TestCase {
                input: "00:11:22:33:44:gg",
                expected: MacAddressError::InvalidHex("gg".to_string()),
            },
            TestCase {
                input: "",
                expected: MacAddressError::InvalidLength(1),
            },
        ];

        for case in cases {
            let err = parse_mac_address(case.input).unwrap_err();
            assert_eq!(
                err.to_string(),
                case.expected.to_string(),
                "input = {}",
                case.input
            );
        }
    }

    #[test]
    fn test_create_magic_packet_structure() {
        let mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];

        let packet = create_magic_packet(&mac);

        assert_eq!(packet.len(), 102);
        assert!(packet[..6].iter().all(|&b| b == 0xFF));

        for i in 0..16 {
            let start = 6 + i * 6;
            let end = start + 6;
            assert_eq!(&packet[start..end], &mac);
        }
    }
}
