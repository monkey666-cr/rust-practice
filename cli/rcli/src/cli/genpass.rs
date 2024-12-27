use clap::Parser;

use crate::{process_genpass, CmdExector};

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub numbers: bool,

    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}

impl CmdExector for GenPassOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let res = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.numbers,
            self.symbols,
        );
        println!("随机生成的密码为: {}", res);
        Ok(())
    }
}
