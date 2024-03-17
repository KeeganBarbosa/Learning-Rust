use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("file_to_read.txt");

    let file = File::open(&path)?;

    let mut best_score = 0;
    let mut best_key = 0;
    let mut best_guess = String::new();
    let mut best_line = String::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let mut index = 0;
        let mut decoded = String::new();
        let mut score = 0;

        for possible_key in 0..255 {
            index = 0;
            decoded = String::new();
            while index < line.len() {
                let hex_byte = &line[index..index + 2];

                if let Ok(byte) = u8::from_str_radix(hex_byte, 16) {
                    decoded.push((byte ^ possible_key) as char);
                    score += score_byte(byte ^ possible_key);

                } else {
                    println!("Error decoding byte.");
                }
                index += 2;
            }

            if score > best_score {
                best_line = line.clone();
                best_guess = decoded.clone();
                best_score = score;
                best_key = possible_key;
                score = 0;
            } else {
                score = 0;
            }
        }
    }

    let best_character = best_key as char;

    println!("The best guess is {}", best_guess);
    println!("It has score {}", best_score);
    println!("It was likely encrypted with the character {}", best_character);
    println!("It was line {}", best_line);

    Ok(())
}

fn score_byte(byte: u8) -> i32 {
    if byte >= 65 && byte <= 90 || byte >= 97 && byte <= 122 {
        return 2;
    } else if byte == 32 {
        return 1;
    } else {
        return -1;
    }
}
