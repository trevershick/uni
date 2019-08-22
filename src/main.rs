use hex;
use std::env;
use std::error::Error;
use std::process::exit;
use std::result::Result;

#[macro_use]
extern crate simple_error;

fn utf8_to_utf16(unicode_bytes: Vec<u8>) -> Result<String, Box<dyn Error>> {
    if unicode_bytes.len() != 2 {
        bail!("only handle arrays of 2");
    }

    let mut wide: u16 = unicode_bytes[0] as u16;
    wide <<= 8;
    wide += unicode_bytes[1] as u16;
    match String::from_utf16(&[wide]) {
        Ok(v) => Ok(v),
        Err(x) => Err(Box::new(x)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (_, hex_values) = args.split_first().unwrap();
    if hex_values.len() < 1 {
        eprintln!("Need at least 1 argument");
        exit(1);
    }

    let mut bad: Vec<String> = Vec::new();

    for hex_value in hex_values {
        let decoded = hex::decode(hex_value);
        match decoded {
            Ok(unicode_bytes) => match utf8_to_utf16(unicode_bytes) {
                Ok(v) => print!("{}", v),
                Err(_) => bad.push(String::from(hex_value)),
            },
            Err(_) => bad.push(String::from(hex_value)),
        };
    }

    if bad.len() > 0 {
        for b in bad {
            eprintln!("bad code {}", b);
        }
        exit(1)
    }
}
