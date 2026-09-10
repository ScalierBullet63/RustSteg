use super::Binary;

#[cfg(debug_assertions)]
use super::ImageMatrix;

pub fn to_byte(bits: &Vec<u8>) -> u8 {
    let mut byte: u8 = 0;
    let mut exp: u8 = 7;
    for bit in bits {
        byte += bit * u8::pow(2, exp as u32);
        exp -= 1;
    }
    byte
}

pub fn to_ascii(bytes: Binary) -> String {
    let mut string = String::new();
    for byte in bytes {
        string.push(byte as char);
    }
    string
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
