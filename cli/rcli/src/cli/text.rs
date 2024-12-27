use core::fmt;
use std::{fs, path::PathBuf, str::FromStr};

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{
    get_content, get_reader, process_text_key_generate, process_text_sign, process_text_verify,
    CmdExector,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use super::{verify_input_file, verify_path};

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a private/session key and return signature")]
    Sign(TextSignOpts),

    #[command(about = "Verify a text with a public/session key and return true/false")]
    Verify(TextVerifyOpts),

    #[command(about = "Generate a random blake3 key or ed25519 key pair")]
    Generate(KeyGenerateOpts),
}

#[derive(Parser, Debug)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_input_file)]
    pub key: String,

    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

impl CmdExector for TextSignOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let mut input_reader = get_reader(&self.input);
        let key = get_content(&self.key).unwrap();

        let res = process_text_sign(&mut input_reader, &key, self.format);
        let encoded = URL_SAFE_NO_PAD.encode(res.unwrap());

        println!("签名结果: {:?}", encoded);

        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_input_file)]
    pub key: String,

    #[arg(short, long)]
    pub signature: String,

    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

impl CmdExector for TextVerifyOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let mut input_reader = get_reader(&self.input);
        let key = get_content(&self.key).unwrap();
        let decoded = URL_SAFE_NO_PAD.decode(&self.signature).unwrap();
        let res = process_text_verify(&mut input_reader, &key, &decoded, self.format).unwrap();
        if res {
            println!("验证成功");
        } else {
            print!("验证失败");
        }

        Ok(())
    }
}

#[derive(Parser, Debug)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

impl CmdExector for KeyGenerateOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let key = process_text_key_generate(self.format).unwrap();
        for (k, v) in key {
            fs::write(self.output_path.join(k), v).unwrap();
        }
        println!("{:?}", self);

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(format!("Invalid text sign format: {}", s)),
        }
    }
}

pub fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, String> {
    format.parse()
}

impl From<TextSignFormat> for &str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<TextSignFormat>::into(*self))
    }
}
