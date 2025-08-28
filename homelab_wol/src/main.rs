mod cli;
mod errors;
mod wol;

use crate::errors::WolError;
use crate::wol::send_magic_packet;

use crate::cli::Options;

fn main() -> Result<(), WolError> {
    let options = Options::new();

    run(options)?;

    Ok(())
}

/// メインエントリポイントの実行
fn run(options: Options) -> Result<(), WolError> {
    println!("MACアドレス = {}", options.mac_address);

    send_magic_packet(options.mac_address)?;

    Ok(())
}
