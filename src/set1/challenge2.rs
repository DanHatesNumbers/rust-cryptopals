extern crate data_encoding;
use self::data_encoding::HEXLOWER;

pub fn solution() {
    let input = b"1c0111001f010100061a024b53535009181c";
    let key = b"686974207468652062756c6c277320657965";
    let expected_output = "746865206b696420646f6e277420706c6179";

    let input_bytes = HEXLOWER.decode(input).unwrap();
    let key_bytes = HEXLOWER.decode(key).unwrap();

    let xored: Vec<u8> = input_bytes.into_iter()
        .zip(key_bytes.into_iter())
        .map(|(input_byte, key_byte)| input_byte ^ key_byte)
        .collect();

    let actual_output = HEXLOWER.encode(&xored);

    assert_eq!(expected_output, actual_output);
    println!("Challenge 2 complete!");
}