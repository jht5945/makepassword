extern crate argparse;
extern crate rand;
extern crate rust_util;

use argparse::{ArgumentParser, StoreTrue, Store};
use rust_util::*;

const VERSION: &str = "0.1";
const CHARS_DIGITALS: &str = "1234567890";
const CHARS_LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const CHARS_UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARS_BASE58: &str = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
const CHARS_SYMBOL: &str = "-_.|!@#$%^&*()+=[]{};:<>";

fn print_version() {
    print!(r#"makepassword {}

Copyright (C) 2019 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>
Written by Hatter Jiang
"#, VERSION);
}

fn main() {
    let mut version = false;
    let mut chars = String::new();
    let mut chars_type = String::from("base58");
    let mut password_count = 1u32;
    let mut password_length = 12u8;
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("makepassword - command line makepassword tool.");
        ap.refer(&mut password_count).add_option(&["-c", "--count"], Store, "Password count, default 1");
        ap.refer(&mut password_length).add_option(&["-l", "--length"], Store, "Password length, default 12");
        ap.refer(&mut chars).add_option(&["--chars"], Store, "Chars, default use -t/--type base58");
        ap.refer(&mut chars_type).add_option(&["-t", "--type"], Store, "Type, base58(default), alphabet, word, all");
        ap.refer(&mut version).add_option(&["-v", "--version"], StoreTrue, "Print version");
        ap.parse_args_or_exit();
    }
    
    if version {
        print_version();
        return;
    }

    if password_count < 1 || password_count > 100 {
        print_message(MessageType::ERROR, &format!("Invalid count: {}", password_count));
        return;
    }

    if password_length < 1 || password_length > 100 {
        print_message(MessageType::ERROR, &format!("Invalid length: {}", password_length));
        return;
    }

    if chars.len() > 0 && chars.len() < 8 {
        print_message(MessageType::ERROR, &format!("Chars too small: {}", &chars));
        return;
    }

    let chars_source: &str = & match chars.len() {
        0 => match chars_type.as_str() {
            "base58" => String::from(CHARS_BASE58),
            "alphabet" => [CHARS_LOWER_CASE, CHARS_UPPER_CASE].join(""),
            "word" => [CHARS_DIGITALS, CHARS_LOWER_CASE, CHARS_UPPER_CASE].join(""),
            "all" => [CHARS_DIGITALS, CHARS_LOWER_CASE, CHARS_UPPER_CASE, CHARS_SYMBOL].join(""),
            _ => {
                print_message(MessageType::ERROR, &format!("Unknown type: {}", chars_type));
                return;
            },
        },
        _ => chars,
    };

    for _ in 0..password_count {
        let mut password = String::new();
        for _ in 0..password_length {
            let p = rand::random::<usize>() % chars_source.len();
            password.push_str(&chars_source[p..p+1]);
        }
        println!("{}", password);
    }
}
