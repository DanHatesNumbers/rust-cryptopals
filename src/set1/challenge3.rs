extern crate ascii;
extern crate data_encoding;
use self::ascii::AsciiString;
use self::data_encoding::HEXLOWER;
use set1::helpers::*;

pub fn solution() {
    let input = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input_bytes: Vec<u8> = HEXLOWER.decode(input).unwrap();

    let candidate_keys: Vec<Vec<u8>> = ascii_printable_chars_bytes();

    let candidate_solution = candidate_keys.iter()
        .map(|x| {
            let candidate_plaintext_result = AsciiString::from_ascii(repeating_key_xor(input_bytes.iter(), x.iter()));
            let candidate_plaintext = candidate_plaintext_result.ok();
            let encoded_key = AsciiString::from_ascii(x.clone()).unwrap();
            let plaintext_score = plaintext_scoring(&candidate_plaintext);
            CandidateSolution {score: plaintext_score, plaintext: candidate_plaintext, key: encoded_key}
        })
        .max_by_key(|x| x.score).unwrap();

    println!("{:?}", candidate_solution);
}
