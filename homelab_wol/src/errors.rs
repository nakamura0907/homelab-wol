use std::io;

use thiserror::Error;

/// Errors related to parsing or validating MAC addresses.
#[derive(Debug, Error)]
pub enum MacAddressError {
    // #[error("Invalid MAC address format: {0}")]
    // InvalidFormat(String),
    /// The MAC address does not have exactly 6 bytes.
    #[error("MAC address must be exactly 6 bytes, got {0} bytes")]
    InvalidLength(usize),

    /// The MAC address contains an invalid hexadecimal value.
    #[error("Invalid hex value in MAC address: {0}")]
    InvalidHex(String),
}

/// Errors related to network operations such as sending packets.
#[derive(Debug, Error)]
pub enum NetworkError {
    /// Failed to create a UDP socket.
    #[error("Failed to create socket: {0}")]
    SocketCreate(#[source] io::Error),

    /// Failed to enable the broadcast flag on the socket.
    #[error("Failed to set broadcast flag: {0}")]
    BroadcastSet(#[source] io::Error),

    /// Failed to send the magic packet.
    #[error("Failed to send packet: {0}")]
    Send(#[source] io::Error),
}

/// General error type for the WOL tool.
#[derive(Debug, Error)]
pub enum WolError {
    #[error("MAC address error: {0}")]
    Mac(#[from] MacAddressError),

    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    // #[error("Unexpected error: {0}")]
    // Unexpected(String),
}
