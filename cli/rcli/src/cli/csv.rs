use std::{fmt::Display, str::FromStr};

use clap::Parser;

use super::verify_input_file;

#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl From<OutputFormat> for &str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl From<&str> for OutputFormat {
    fn from(format: &str) -> Self {
        match format {
            "json" => OutputFormat::Json,
            "yaml" => OutputFormat::Yaml,
            _ => panic!("Unsupported format"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(format!("Unsupported format: {}", value)),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = false)]
    pub header: bool,
}

fn parse_format(format: &str) -> Result<OutputFormat, String> {
    format.to_lowercase().parse()
}
