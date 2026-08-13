type Binary = Vec<u8>;

pub struct Payload {
    plain_text: String,
    binary: Binary,
}

impl Payload {
    pub fn new() -> Self {
        Self {
            plain_text: String::new(),
            binary: Binary::new(),
        }
    }

    pub fn set_plain_text(&mut self, plain_text: String) {
        self.convert_binary(&plain_text);
        self.plain_text = plain_text;
    }

    pub fn _set_binary(&mut self, binary: Binary) {
        self.binary = binary;
        todo!("Add binary decoding");
    }

    pub fn plain_text(&self) -> &str {
        &self.plain_text
    }

    pub fn binary(&self) -> &Vec<u8> {
        &self.binary
    }

    fn convert_binary(&mut self, hidden_message: &str) {
        let mut hidden_bits: Binary = Vec::new();
        let mut binary: String = String::new();

        for char in hidden_message.to_string().into_bytes() {
            binary += &format!("0{:b}", char);
        }

        for bit in binary.chars() {
            hidden_bits.push(bit.to_digit(2).unwrap() as u8);
        }

        self.binary = hidden_bits;
    }
}
