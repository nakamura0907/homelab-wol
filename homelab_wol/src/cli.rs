use clap::Parser;

/// CLI options for the HomeLab Wake-on-LAN tool.
///
/// This struct defines the arguments that can be passed via the command line.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Options {
    /// The target device's MAC address to send the magic packet to.
    pub mac_address: String,
}

impl Options {
    /// Parse CLI arguments into an [`Options`] instance.
    pub fn new() -> Self {
        let options = Self::parse();

        options
    }
}
