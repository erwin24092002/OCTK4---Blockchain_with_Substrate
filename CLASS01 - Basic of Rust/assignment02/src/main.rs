/*
Cho 1 chuỗi ký tự, nhập 1 ký tự từ bàn phím trả về số lần xuất hiện của từ đó trong chuỗi đã cho, 
và chuỗi không chứa ký tự nhập từ bàn phím. Lưu ý: khong phân biệt viết hoa, viết thường
Ví dụ: let input = “adbcdaDd”. 
Nhập s = ‘a’ => in ra kết quả : 2, “dbcdDd”
Nhập s = ‘d’ => in ra kết quả : 4, “abca”
*/
#![allow(unused)]
use std::{io::{self, Read}, collections::{HashMap}};

fn main() {
    println!("Enter the string: ");
    let mut input_s = String::new();
    io::stdin().read_line(&mut input_s).expect("No input");
    input_s = input_s.to_lowercase();

    println!("Enter a character: ");
    let mut find_c = String::new();
    io::stdin().read_line(&mut find_c).expect("No input");
    find_c = find_c.to_lowercase();

    let mut rest_s: String = String::new();

    let mut count: i32 = 0;
    for inp_idx in 0..(input_s.len()-2)
    {
        if input_s.chars().nth(inp_idx) == find_c.chars().nth(0)
        {
            count = count + 1;
        }
        else 
        {
            rest_s.push(input_s.chars().nth(inp_idx).unwrap()); 
        }
    }
    print!("Output: {}, {}", count, rest_s);

}