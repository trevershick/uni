#[macro_use]
extern crate simple_error;
#[macro_use]
extern crate clap;

use std::error::Error;
use std::process::exit;
use std::result::Result;

use clap::App;
use hex;

fn utf8_to_utf16(unicode_bytes: Vec<u8>) -> Result<String, Box<dyn Error>> {
    if unicode_bytes.len() != 2 {
        bail!("only handle arrays of 2");
    }

    let mut wide = unicode_bytes[0] as u16;
    wide <<= 8;
    wide += unicode_bytes[1] as u16;
    return match String::from_utf16(&[wide]) {
        Ok(v) => Ok(v),
        Err(x) => Err(Box::new(x)),
    };
}

fn main() {
    let matches = App::new("uni")
        .version("1.0.0")
        .about("Convert unicode hex to unicode character")
        .args_from_usage("<hex_vals>... 'A sequence of utf16 hex values, i.e. 30CE B0AB'")
        .get_matches();

    let hex_values = values_t!(matches, "hex_vals", String).unwrap();

    let mut bad: Vec<String> = Vec::new();
    for hex_value in hex_values {
        let decoded = hex::decode(&hex_value);
        match decoded {
            Ok(unicode_bytes) => match utf8_to_utf16(unicode_bytes) {
                Ok(v) => print!("{}", v),
                Err(_) => bad.push(hex_value),
            },
            Err(_) => bad.push(hex_value),
        };
    }

    for b in &bad {
        eprintln!("bad code {}", b);
    }
    if bad.len() > 0 {
        exit(1)
    }
}
