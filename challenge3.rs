fn main() {
    //let key : char = '0';

    //let hex_key = key as u128;

    let encoded_1:i32 = 0x1b373733;
    let encoded_2:i32 = 0x31363f78;
    let encoded_3:i32 = 0x151b7f2b;
    let encoded_4:i32 = 0x78343133;
    let encoded_5:i32 = 0x3d783978;
    let encoded_6:i32 = 0x28372d36;
    let encoded_7:i32 = 0x3c78373e;
    let encoded_8:i32 = 0x783a393b;
    let encoded_9:i32 = 0x37360000;

    let bytes_1: [u8;4] = encoded_1.to_be_bytes();
    let bytes_2: [u8;4] = encoded_2.to_be_bytes();
    let bytes_3: [u8;4] = encoded_3.to_be_bytes();
    let bytes_4: [u8;4] = encoded_4.to_be_bytes();
    let bytes_5: [u8;4] = encoded_5.to_be_bytes();
    let bytes_6: [u8;4] = encoded_6.to_be_bytes();
    let bytes_7: [u8;4] = encoded_7.to_be_bytes();
    let bytes_8: [u8;4] = encoded_8.to_be_bytes();
    let bytes_9: [u8;4] = encoded_9.to_be_bytes();

    for possible_key in 0..255{
        for chunk in [bytes_1,bytes_2,bytes_3,bytes_4,bytes_5,bytes_6,bytes_7,bytes_8,bytes_9].iter(){
            for byte in chunk.iter() {
            print!("{}", (*byte^possible_key) as char);
            }
        }
        println!("");
        println!("{}",possible_key as char);
        println!("");
    }


}
