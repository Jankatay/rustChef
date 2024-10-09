#![allow(non_snake_case)]

pub fn main() {
    let bitVec : Vec<u8> = bitVector("hello, world");
    for i in bitVec {
        print!("0{:b}", i);
    }
}

fn bitVector(targetString : &str) -> Vec<u8> {
    let mut bitVec : Vec<u8> = Vec::new();
    
    for character in str::chars(targetString) {
        let charAscii : u8 = character as u8; 
        for i in (0..8).rev() {
            Vec::push(&mut bitVec, (charAscii>>i) & 1);
        }
    }
    
    return bitVec;
}
