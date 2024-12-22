use rand::seq::SliceRandom;

use crate::cli::GenPassOpts;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

pub fn process_genpass(opts: GenPassOpts) -> String {
    let mut rng = rand::thread_rng();
    let mut password = vec![];
    let mut chars = vec![];

    if opts.uppercase {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if opts.lowercase {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if opts.numbers {
        chars.extend(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).expect("NUMBERS won't be empty"));
    }
    if opts.symbols {
        chars.extend(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).expect("SYMBOLS won't be empty"));
    }

    if opts.length > password.len() as u8 {
        for _ in 0..(opts.length - password.len() as u8) {
            let c = chars.choose(&mut rng).expect("chars won't be empty");
            password.push(*c);
        }
    }

    password.shuffle(&mut rng);

    String::from_utf8(password[..opts.length as usize].to_vec()).unwrap()
}
