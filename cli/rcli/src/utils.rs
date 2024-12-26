use anyhow::Result;
use std::{fs::File, io::Read};

pub fn get_reader(input: &str) -> Box<dyn Read> {
    if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input).unwrap())
    }
}

pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
