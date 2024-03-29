extern crate argparse;
extern crate rand;
#[macro_use]
extern crate rust_util;

use argparse::{ArgumentParser, Store, StoreTrue};

const CHARS_DIGITALS: &str = "1234567890";
const CHARS_LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const CHARS_UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARS_BASE58: &str = "123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
const CHARS_SYMBOL: &str = "-_.|!@#$%^&*()+=[]{};:<>";

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");

fn print_version() {
    print!(r#"makepassword {} - {}
Copyright (C) 2019-2022 Hatter Jiang.
License MIT <https://opensource.org/licenses/MIT>

Written by Hatter Jiang
"#, VERSION, &GIT_HASH[0..7]);
}

struct Options {
    version: bool,
    chars: String,
    chars_type: String,
    password_count: u32,
    password_length: u8,
}

fn main() {
    let mut options = Options {
        version: false,
        chars: String::new(),
        chars_type: String::from("base58"),
        password_count: 1u32,
        password_length: 12u8,
    };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("makepassword - command line makepassword tool.");
        ap.refer(&mut options.password_count).add_option(&["-c", "--count"], Store, "Password count, default 1");
        ap.refer(&mut options.password_length).add_option(&["-l", "--length"], Store, "Password length, default 12");
        ap.refer(&mut options.chars).add_option(&["--chars"], Store, "Chars, default use -t/--type base58");
        ap.refer(&mut options.chars_type).add_option(&["-t", "--type"], Store, "Type, base58(default, b58, b), alphabet(a), word(w), all(A)");
        ap.refer(&mut options.version).add_option(&["-v", "--version"], StoreTrue, "Print version");
        ap.parse_args_or_exit();
    }

    if options.version {
        print_version();
        return;
    }

    if options.password_count < 1 || options.password_count > 100 {
        failure!("Invalid count: {}", options.password_count);
        return;
    }

    if options.password_length < 1 || options.password_length > 100 {
        failure!("Invalid length: {}", options.password_length);
        return;
    }

    if !options.chars.is_empty() && options.chars.len() < 8 {
        failure!("Chars too small: {}", &options.chars);
        return;
    }

    let chars_source: &str = &match options.chars.len() {
        0 => match options.chars_type.as_str() {
            "base58" | "b58" | "b" => String::from(CHARS_BASE58),
            "alphabet" | "a" => [CHARS_LOWER_CASE, CHARS_UPPER_CASE].join(""),
            "word" | "w" => [CHARS_DIGITALS, CHARS_LOWER_CASE, CHARS_UPPER_CASE].join(""),
            "all" | "A" => [CHARS_DIGITALS, CHARS_LOWER_CASE, CHARS_UPPER_CASE, CHARS_SYMBOL].join(""),
            _ => {
                failure!("Unknown type: {}", options.chars_type);
                return;
            }
        },
        _ => options.chars,
    };

    for _ in 0..options.password_count {
        let mut password = String::new();
        for _ in 0..options.password_length {
            let p = rand::random::<usize>() % chars_source.len();
            password.push_str(&chars_source[p..p + 1]);
        }
        println!("{}", password);
    }
}
