extern crate data_encoding;
use self::data_encoding::{BASE64, HEXLOWER};

pub fn solution() {
    let input = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let actual_output = BASE64.encode(&HEXLOWER.decode(input).unwrap());

    assert_eq!(expected_output, actual_output);
}