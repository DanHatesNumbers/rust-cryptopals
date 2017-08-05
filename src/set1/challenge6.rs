extern crate ascii;
extern crate data_encoding;
extern crate hamming;
use self::ascii::{AsciiString, AsciiStr, AsciiChar, FromAsciiError};
use self::data_encoding::BASE64;
use self::hamming::distance;
use std::ascii::AsciiExt;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;
use set1::helpers::*;

pub fn solution() {
    let raw_input = read_input();
    match raw_input {
        Err(e) => {
            panic!("Error decoding string to ASCII, {:?}", e.description());
        }
        Ok(input) => {
            let filtered_input = input.chars()
                .filter(|x| match **x {
                    AsciiChar::LineFeed => false,
                    AsciiChar::CarriageReturn => false,
                    _ => true
                })
                .cloned()
                .collect::<AsciiString>();

            let decoded_input = BASE64.decode(filtered_input.as_bytes());
            match decoded_input {
                Err(e) => panic!("Error decoding input as Base64, {:?}", e),
                Ok(ciphertext) => {
                    let guessed_keysize = guess_keysize(&ciphertext);
                    println!("Guessed keysize: {:?}", guessed_keysize);
                    match guessed_keysize {
                        Some(keysize) => {
                            let ascii_key_bytes = ascii_printable_chars_bytes();
                        },
                        None => panic!("Couldn't guess keysize")
                    }
                }
            }
        }
    }
}

fn read_input() -> Result<AsciiString, FromAsciiError<Vec<u8>>> {
    use std::io::Read;

    let base_path = Path::new(".");
    let path = base_path.join("src").join("set1").join("data").join("challenge6.txt");
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    let result = BufReader::new(file).read_to_end(&mut buffer);
    if result.is_err() {
        panic!("Error reading file");
    }
    AsciiString::from_ascii(buffer)
}

fn guess_keysize(ciphertext: &[u8]) -> Option<u64> {
    let guessed_keysize = (2..41).map(|keysize| {
        let chunks = ciphertext.chunks(keysize).take(4).collect::<Vec<_>>();
        let keysize = keysize as u64;

        let first_distance = hamming::distance(chunks.get(0).unwrap(), chunks.get(1).unwrap());
        let second_distance = hamming::distance(chunks.get(2).unwrap(), chunks.get(3).unwrap());
        let avg_distance: u64 = (first_distance / keysize) / (second_distance / keysize);
        (keysize, avg_distance)
    })
    .max_by_key(|x| x.1);

    match guessed_keysize {
        Some((keysize, _)) => Some(keysize),
        None => None
    }
}

#[cfg(test)]
mod challenge_6_tests {
    extern crate hamming;
    use self::hamming::distance;
    #[test]
    fn hamming_distance_correct() {
        let first = b"this is a test";
        let second = b"wokka wokka!!!";

        assert_eq!(distance(first, second), 37);
    }
}