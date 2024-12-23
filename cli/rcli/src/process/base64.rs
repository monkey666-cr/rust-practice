use std::io::Read;

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::cli::Base64Format;

pub fn process_encode(reader: &mut dyn Read, format: Base64Format) -> String {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();

    match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    }
}

pub fn process_decode(reader: &mut dyn Read, format: Base64Format) -> String {
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf).unwrap(),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf).unwrap(),
    };

    String::from_utf8(decode).unwrap()
}
