use clap::Parser;

/// HomeLab WoLツールのCLIオプション
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Options {
    /// 対象のMACアドレス
    pub mac_address: String,
}

impl Options {
    pub fn new() -> Self {
        let options = Self::parse();

        options
    }
}
