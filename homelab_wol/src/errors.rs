use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MacAddressError {
    // #[error("Invalid MAC address format: {0}")]
    // InvalidFormat(String),
    #[error("MAC address must be exactly 6 bytes, got {0} bytes")]
    InvalidLength(usize),

    #[error("Invalid hex value in MAC address: {0}")]
    InvalidHex(String),
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Failed to create socket: {0}")]
    SocketCreate(#[source] io::Error),

    #[error("Failed to set broadcast flag: {0}")]
    BroadcastSet(#[source] io::Error),

    #[error("Failed to send packet: {0}")]
    Send(#[source] io::Error),
}

#[derive(Debug, Error)]
pub enum WolError {
    #[error("MAC address error: {0}")]
    Mac(#[from] MacAddressError),

    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    // #[error("Unexpected error: {0}")]
    // Unexpected(String),
}
