use std::collections::HashMap;

fn vectorize(s: &str) -> Vec<u8> {
    
    let mut out = Vec::new();
    
    let hex_to_bin_dict: HashMap<char, u8> = [
        ('0', 0b0000),
        ('1', 0b0001),
        ('2', 0b0010),
        ('3', 0b0011),
        ('4', 0b0100),
        ('5', 0b0101),
        ('6', 0b0110),
        ('7', 0b0111),
        ('8', 0b1000),
        ('9', 0b1001),
        ('a', 0b1010),
        ('b', 0b1011),
        ('c', 0b1100),
        ('d', 0b1101),
        ('e', 0b1110),
        ('f', 0b1111),
    ]
    .iter()
    .cloned()
    .collect();
    for chars in s.chars() {
        match hex_to_bin_dict.get(&chars) {
            Some(&bin_data) => out.push(bin_data),
            None => println!("Cannot convert hex to bin."),
        }

    }
    out
}

fn coord_xor(bin_vec:Vec<u8>,shift:char) -> Vec<u8> {
    
    let byte = shift as u8;
    
    let mut out = Vec::new();
    
    for bin in bin_vec.iter() {
        out.push(bin^byte)
    }
    
    out
}

fn main() {
    println!("{:?}",coord_xor(vectorize("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"),'\u{0000}'))
}