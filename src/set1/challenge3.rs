extern crate data_encoding;
use self::data_encoding::HEXLOWER;

pub fn solution() {
    let input = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input_bytes: Vec<u8> = HEXLOWER.decode(input).unwrap();

    let candidate_keys: Vec<Vec<u8>> = (0..26).map(|x| vec![x + 'a' as u8]).collect::<Vec<Vec<_>>>();

    let candidate_solutions = candidate_keys.iter()
        .map(|x| String::from_utf8(repeating_key_xor(input_bytes.iter(), x.iter())))
        .collect::<Vec<_>>();
}

fn repeating_key_xor<'a, 'b, T, U>(input_bytes: T, key_bytes: U) -> Vec<u8> 
where T: Iterator<Item=&'a u8>, U: Iterator<Item=&'b u8> + Clone
{
    input_bytes
        .zip(key_bytes.cycle())
        .map(|(input_byte, key_byte)| input_byte ^ key_byte)
        .collect::<Vec<_>>()
}
