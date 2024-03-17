use std::io;
use std::collections::HashMap;

fn to_bin(s : &str) -> String {
    let hex_to_bin_dict: HashMap<char, &str> = [
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut final_bin = String::new();

    for c in s.to_uppercase().chars() {
        match hex_to_bin_dict.get(&c) {
            Some(bin_data) => final_bin.push_str(bin_data),
            None => println!("Invalid hexadecimal digit: {}", c),
        }
    }
    final_bin
}

fn left_pad(s: &str) -> String {
    let mut padded = s.to_string();
    while padded.len() % 6 != 0 {
        padded.insert(0, '0');
    }
    padded
}


fn bin_to_64(s : &str) -> String {
    
    let mut base_64_out = String::new();


    
    let bin_to_base64_dict: HashMap<&str, &str> = [
        ("000000", "A"), ("000001", "B"), ("000010", "C"), ("000011", "D"), 
        ("000100", "E"), ("000101", "F"), ("000110", "G"), ("000111", "H"),
        ("001000", "I"), ("001001", "J"), ("001010", "K"), ("001011", "L"),
        ("001100", "M"), ("001101", "N"), ("001110", "O"), ("001111", "P"),
        ("010000", "Q"), ("010001", "R"), ("010010", "S"), ("010011", "T"),
        ("010100", "U"), ("010101", "V"), ("010110", "W"), ("010111", "X"),
        ("011000", "Y"), ("011001", "Z"), ("011010", "a"), ("011011", "b"),
        ("011100", "c"), ("011101", "d"), ("011110", "e"), ("011111", "f"),
        ("100000", "g"), ("100001", "h"), ("100010", "i"), ("100011", "j"),
        ("100100", "k"), ("100101", "l"), ("100110", "m"), ("100111", "n"),
        ("101000", "o"), ("101001", "p"), ("101010", "q"), ("101011", "r"),
        ("101100", "s"), ("101101", "t"), ("101110", "u"), ("101111", "v"),
        ("110000", "w"), ("110001", "x"), ("110010", "y"), ("110011", "z"),
        ("110100", "0"), ("110101", "1"), ("110110", "2"), ("110111", "3"),
        ("111000", "4"), ("111001", "5"), ("111010", "6"), ("111011", "7"),
        ("111100", "8"), ("111101", "9"), ("111110", "+"), ("111111", "/"),
    ]
    .iter()
    .cloned()
    .collect();

    for chunk in s.chars().collect::<Vec<char>>().chunks(6) {
        
        match bin_to_base64_dict.get(&chunk.iter().collect::<String>().as_str()) {
            Some(bin_data) => base_64_out.push_str(bin_data),
            None => println!("Invalid binary chunk: {}", chunk.iter().collect::<String>().as_str()),
        }
    }
    base_64_out
}

fn main() {
    println!("Input hex string");

    let mut hex_string = String::new();

    io::stdin()
        .read_line(&mut hex_string)
        .expect("Failed to read line");
    
    let binary_rep = to_bin(&hex_string);

    let padded_binary = left_pad(&binary_rep);

    let output = bin_to_64(&padded_binary);

    println!("Output: {}", output);
}