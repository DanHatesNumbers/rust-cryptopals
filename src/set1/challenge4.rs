extern crate ascii;
extern crate data_encoding;
use self::ascii::{AsciiString, AsciiStr, AsciiChar};
use self::data_encoding::HEXLOWER;
use std::ascii::AsciiExt;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;

pub fn solution() {
    let input_strings = read_input_strings();
}

fn read_input_strings() -> Vec<AsciiString> {
    let base_path = Path::new(".");
    let path = base_path.join("src").join("set1").join("data").join("challenge4.txt");
    let mut file = File::open(path).unwrap();
    BufReader::new(file).lines().map(|x| AsciiString::from_ascii(x.unwrap()).unwrap()).collect::<Vec<_>>()
}