#![allow(unused)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Error: {0}")]
    Other(String),

    #[error("Custom error: {0}")]
    CustomError(String),

    #[error("Big error: {0:?}")]
    BigError(Box<BigError>),

    // #[error("Error: {a}, {b:?}, {c:?}, {d:?}")]
    // BigError {
    //     a: String,
    //     b: Vec<String>,
    //     c: [u8; 64],
    //     d: u64,
    // },
}

#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 64],
    d: u64,
}
