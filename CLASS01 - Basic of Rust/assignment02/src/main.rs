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

    println!("Enter the string to be counted: ");
    let mut find_s = String::new();
    io::stdin().read_line(&mut find_s).expect("No input");
    
    let mut count: i32 = 0;
    for inp_idx in 0..(input_s.len()-find_s.len()+1)
    {
        let mut flag = true;
        for fid_idx in 0..(find_s.len()-2)
        {
            if input_s.chars().nth(inp_idx+fid_idx) != find_s.chars().nth(fid_idx)
            {
                flag = false;
            }
        }
        if flag 
        {
            count = count + 1;
        }
    }
    print!("Output: {}, {}", count, find_s);

}