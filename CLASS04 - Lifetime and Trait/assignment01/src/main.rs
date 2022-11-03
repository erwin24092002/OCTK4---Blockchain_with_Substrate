// ///////////////////////////////////
// Bài 1
// Sửa code để compile thành công liên quan tới vấn đề lifetime
// ///////////////////////////////////

#![allow(unused)]
use std::io;
fn main() {
    let mut input: Vec<String>;
    loop {
        let mut input_text = String::new();
        println!("Type instruction in the format Add <name> to <department>:");
        io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
        let trimmed_text = input_text.trim().to_string();
        input = trimmed_text.split(" ").map(String::from).collect();
        if input[0] == "Add" && input[2] == "to" {
            break;
        } else {
            println!("Invalid format.");
        }
    }
    println!("{:?}", input);
}