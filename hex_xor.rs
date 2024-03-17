// use std::io;
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

fn string_xor(left : &str, right : &str) -> String {
    
    let mut xor_string = String::new();

    if left.len() == right.len() {
        for i in 0..left.len(){
            if left.chars().nth(i) == right.chars().nth(i) {
                xor_string.push_str("0")
            }
            else {
                xor_string.push_str("1")
            }
        }
    }
    else {
        println!("Strings are not of same length")
    }

    xor_string
}

fn left_pad(s: &str) -> String {
    let mut padded = s.to_string();
    while padded.len() % 4 != 0 {
        padded.insert(0, '0');
    }
    padded
}

fn bin_to_hex(s : &str) -> String {
    
    let mut hex_out = String::new();


    
    let bin_to_hex_dict: HashMap<&str, &str> = [
        ("0000", "0"),
        ("0001", "1"),
        ("0010", "2"),
        ("0011", "3"),
        ("0100", "4"),
        ("0101", "5"),
        ("0110", "6"),
        ("0111", "7"),
        ("1000", "8"),
        ("1001", "9"),
        ("1010", "A"),
        ("1011", "B"),
        ("1100", "C"),
        ("1101", "D"),
        ("1110", "E"),
        ("1111", "F"),
    ]
    .iter()
    .cloned()
    .collect();

    for chunk in s.chars().collect::<Vec<char>>().chunks(4) {
        
        match bin_to_hex_dict.get(&chunk.iter().collect::<String>().as_str()) {
            Some(bin_data) => hex_out.push_str(bin_data),
            None => println!("Invalid binary chunk: {}", chunk.iter().collect::<String>().as_str()),
        }
    }
    hex_out
}

fn main() {



    let hex_1 = "1c0111001f010100061a024b53535009181c";

    let hex_2 = "686974207468652062756c6c277320657965";

    let bin_rep_1 = to_bin(&hex_1);

    let bin_rep_2 = to_bin(&hex_2);

    let binary_output = string_xor(&bin_rep_1,&bin_rep_2);

    let padded_binary_output = left_pad(&binary_output);

    let output = bin_to_hex(&padded_binary_output);

    println!("{}",output)

}