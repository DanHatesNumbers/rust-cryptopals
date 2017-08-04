extern crate ascii;
extern crate data_encoding;
use self::ascii::{AsciiString, AsciiStr, AsciiChar};
use self::data_encoding::HEXLOWER;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;
use set1::helpers::*;

pub fn solution() {
    let input_strings = read_input_strings();
    let candidiate_keys = ascii_printable_chars_bytes();
    let solutions: HashMap<_,_> = input_strings.iter()
        .map(|input| {
            let input_bytes = HEXLOWER.decode(input.as_bytes()).expect("Error decoding input");
            let solutions = candidiate_keys.iter()
                .map(|key| {
                    let candidate_plaintext_result = AsciiString::from_ascii(repeating_key_xor(input_bytes.iter(), key.iter()));
                    let candidate_plaintext = candidate_plaintext_result.ok();
                    let encoded_key = AsciiString::from_ascii(key.clone()).expect("Error encoding key");
                    let plaintext_score = plaintext_scoring(&candidate_plaintext);
                    CandidateSolution {score: plaintext_score, plaintext: candidate_plaintext, key: encoded_key}
                })
                .collect::<Vec<_>>();
            (input, solutions)
        })
        .collect();
    
    let encrypted_input = solutions.iter()
        .max_by_key(|&(input, solutions)| 
            solutions.iter()
                .max_by_key(|solution| solution.score)
        );
    
    match encrypted_input {
        Some(input) => {
            let solution = input.1.iter().max_by_key(|x| x.score);
            println!("Input: {:?}, Solution: {:?}", input.0, solution);
        }
        None => println!("Couldn't find solution")
    }
}

fn read_input_strings() -> Vec<AsciiString> {
    let base_path = Path::new(".");
    let path = base_path.join("src").join("set1").join("data").join("challenge4.txt");
    let mut file = File::open(path).unwrap();
    BufReader::new(file).lines().map(|x| AsciiString::from_ascii(x.unwrap()).unwrap()).collect::<Vec<_>>()
}