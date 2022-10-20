#![allow(unused)]

use std::{io::{self, Read}, collections::{HashMap}};

fn main() {
    println!("Enter a string: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("No input");

    let mut mapping: HashMap<String, i32> = HashMap::new();
    let input_char_vec: Vec<char> = input.chars().collect();

    for ch in input_char_vec {
        if ch != '\n' && ch != '\r' {
            mapping.entry(ch.to_lowercase().to_string())
                    .and_modify(|num| *num += 1)
                    .or_insert(1);
        }
    }

    loop {
        println!("Enter a character: ");
        let mut char = String::new();
        io::stdin().read_line(&mut char).expect("No input");
        match mapping.get(&char[0..1].to_lowercase().to_string()) {
            Some(number) => println!("{}: {}", &char[0..1], number),
            None => println!("{} not found.", &char[0..1])
        }
    }
}