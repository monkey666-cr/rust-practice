use core::fmt;
use std::str::FromStr;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{get_reader, process_decode, process_encode, CmdExector};

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExector)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    #[arg(short, long, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExector for Base64EncodeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input);
        let res = process_encode(&mut reader, self.format);

        println!("编码结果: {}", res);

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "url-safe" => Ok(Base64Format::UrlSafe),
            _ => Err("Invalid format".to_string()),
        }
    }
}

impl From<Base64Format> for &str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "url-safe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    #[arg(short, long, default_value = "-")]
    pub input: String,
    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExector for Base64DecodeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input);
        let res = process_decode(&mut reader, self.format);

        println!("解码结果: {}", res);

        Ok(())
    }
}

fn parse_base64_format(format: &str) -> Result<Base64Format, String> {
    format.parse()
}
