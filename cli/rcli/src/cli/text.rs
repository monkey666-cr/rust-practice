use core::fmt;
use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use super::{verify_input_file, verify_path};

#[derive(Parser, Debug)]
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

#[derive(Parser, Debug)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
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
