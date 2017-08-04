extern crate data_encoding;
use self::data_encoding::HEXLOWER;
use set1::helpers::*;

pub fn solution() {
    let input = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";

    let key = b"ICE";

    let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    let actual_output = HEXLOWER.encode(&repeating_key_xor(input.iter(), key.iter()));

    assert_eq!(expected_output, actual_output);
    println!("Challenge 5 complete!");
}