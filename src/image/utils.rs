use super::Bytes;

#[cfg(debug_assertions)]
use super::ImageMatrix;

pub fn to_bits(bytes: &[u8]) -> Vec<u8> {
    //Vec of bits (Big Endian)
    let bits: Vec<u8> = bytes
        .iter()
        .flat_map(|byte| (0..8).rev().map(move |i| (byte >> i) & 1))
        .collect();

    bits
}

pub fn to_bytes(bits: &[u8]) -> Bytes {
    let (chunks, _remainder) = bits.as_chunks::<8>();
    let bytes: Bytes = chunks.iter().map(to_byte).collect();
    bytes
}

fn to_byte(bits: &[u8; 8]) -> u8 {
    let mut byte: u8 = 0;
    let mut exp: u8 = 7;
    for bit in bits {
        byte += bit * u8::pow(2, exp as u32);
        exp -= 1;
    }
    byte
}

#[cfg(debug_assertions)]
pub fn debug_image(img: &ImageMatrix) {
    for y in img.iter() {
        for pixel in y {
            print!("Red: {:?} ", pixel[0]);
            print!("Green: {:?} ", pixel[1]);
            println!("Blue: {:?}", pixel[2]);
        }
        println!();
    }
}
