use std::str::Bytes;

pub fn convert_binary(hidden_message: String) {
    let mut hidden_bits: Vec<u8> = Vec::new();

    let bytes: Bytes = hidden_message.bytes();

    for byte in bytes {
        let binary: String = format!("{byte:b}");
        for bit in binary.as_str().chars() {
            hidden_bits.push(bit.to_digit(2).unwrap() as u8);
        }
    }
    dbg!(hidden_bits);
}
