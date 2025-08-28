mod cli;
mod errors;
mod wol;

use crate::errors::WolError;
use crate::wol::send_magic_packet;

use crate::cli::Options;

fn main() -> Result<(), WolError> {
    // Parse command-line options
    let options = Options::new();

    run(options)?;

    Ok(())
}

/// Runs the main application logic.
///
/// This function sends a WOL magic packet.
fn run(options: Options) -> Result<(), WolError> {
    println!("Target MAC address = {}", options.mac_address);

    send_magic_packet(options.mac_address)?;

    Ok(())
}
