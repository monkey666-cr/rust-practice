use std::io::Read;

use ed25519_dalek::{ed25519::signature::SignerMut, SigningKey};

use crate::cli::TextSignFormat;

pub fn process_sign(input: &mut dyn Read, key: &mut dyn Read, format: TextSignFormat) -> String {
    let mut key_buf = Vec::new();
    key.read_to_end(&mut key_buf).unwrap();
    let mut input_buf = Vec::new();
    input.read_to_end(&mut input_buf).unwrap();

    match format {
        TextSignFormat::Blake3 => {
            let res = blake3::keyed_hash(&key_buf[..32].try_into().unwrap(), &input_buf);
            res.to_string()
        }
        TextSignFormat::Ed25519 => {
            let mut key: SigningKey = key_buf[..32].try_into().unwrap();
            key.sign(&input_buf).to_string()
        }
    }
}

pub fn process_verify(
    input: &mut dyn Read,
    key: &mut dyn Read,
    signature: &mut dyn Read,
    format: TextSignFormat,
) -> bool {
    let mut input_buf = Vec::new();
    input.read_to_end(&mut input_buf).unwrap();
    let mut key_buf = Vec::new();
    key.read_to_end(&mut key_buf).unwrap();
    let mut signature_buf = String::new();
    signature.read_to_string(&mut signature_buf).unwrap();

    match format {
        TextSignFormat::Blake3 => {
            let sign = blake3::keyed_hash(key_buf[..32].try_into().unwrap(), &input_buf);

            sign.to_string() == signature_buf
        }
        TextSignFormat::Ed25519 => {
            // TODO
            true
        }
    }
}

pub fn process_generate() {}
