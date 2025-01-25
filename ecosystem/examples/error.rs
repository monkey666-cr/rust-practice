#![allow(unused)]

use anyhow::{Context, Error};
use ecosystem::error::MyError;
use std::fs;

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    println!(
        "size of anyhow::Error is {}",
        std::mem::size_of::<anyhow::Error>()
    );
    println!(
        "size of MyError::CustomError is {}",
        std::mem::size_of::<MyError>()
    );

    // let filename = "non_existent_file.txt";
    // let _fd =
    //     fs::File::open(filename).with_context(|| format!("can not find file: {}", filename))?; // This will return an error

    let _result = fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::CustomError("This is a custom error".to_string()))
}
