use std::error::Error;

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

    Ok(())
}
