extern crate ascii;
extern crate data_encoding;
use self::ascii::{AsciiString, AsciiStr, AsciiChar};
use self::data_encoding::HEXLOWER;
use std::ascii::AsciiExt;

pub fn solution() {
    let input = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input_bytes: Vec<u8> = HEXLOWER.decode(input).unwrap();

    let candidate_keys: Vec<Vec<u8>> = (32..126).map(|x| vec![x as u8]).collect::<Vec<Vec<_>>>();

    let candidate_solution = candidate_keys.iter()
        .map(|x| {
            let candidate_plaintext = AsciiString::from_ascii(repeating_key_xor(input_bytes.iter(), x.iter())).unwrap();
            let encoded_key = AsciiString::from_ascii(x.clone()).unwrap();
            let plaintext_score = plaintext_scoring(&candidate_plaintext);
            CandidateSolution {score: plaintext_score, plaintext: candidate_plaintext, key: encoded_key}
        })
        .max_by_key(|x| x.score).unwrap();

    println!("{:?}", candidate_solution);
}

fn repeating_key_xor<'a, 'b, T, U>(input_bytes: T, key_bytes: U) -> Vec<u8> 
where T: Iterator<Item=&'a u8>, U: Iterator<Item=&'b u8> + Clone
{
    input_bytes
        .zip(key_bytes.cycle())
        .map(|(input_byte, key_byte)| input_byte ^ key_byte)
        .collect::<Vec<_>>()
}

fn plaintext_scoring(plaintext: &AsciiStr) -> isize {
    plaintext
        .to_ascii_lowercase()
        .chars()
        .map(|x| 
        {
            match *x
            {
                AsciiChar::e => 26,
                AsciiChar::t => 25,
                AsciiChar::a => 24,
                AsciiChar::o => 23,
                AsciiChar::i => 22,
                AsciiChar::n => 21,
                AsciiChar::s => 20,
                AsciiChar::r => 19,
                AsciiChar::h => 18,
                AsciiChar::l => 17,
                AsciiChar::d => 16,
                AsciiChar::c => 15,
                AsciiChar::u => 14,
                AsciiChar::m => 13,
                AsciiChar::f => 12,
                AsciiChar::p => 11,
                AsciiChar::g => 10,
                AsciiChar::w => 9,
                AsciiChar::y => 8,
                AsciiChar::b => 7,
                AsciiChar::v => 6,
                AsciiChar::k => 5,
                AsciiChar::x => 4,
                AsciiChar::j => 3,
                AsciiChar::q => 2,
                AsciiChar::z => 1,
                AsciiChar::Space => 1,
                _ => -1
            }
        })
    .sum()
}

#[derive(Debug)]
struct CandidateSolution
{
    score: isize,
    key: AsciiString,
    plaintext: AsciiString
}

