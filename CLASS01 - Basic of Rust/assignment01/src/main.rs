/*
Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
let sub_arr = [6,8,10];
 */
#![allow(unused)]
fn main() {
    let org_arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sub_arr: [i32; 3] = [4, 8, 7];

    let mut org_arr_idx = 0;
    let mut sub_arr_idx = 0;
    while org_arr_idx < org_arr.len()
    {
        if (sub_arr_idx < sub_arr.len() && org_arr[org_arr_idx] == sub_arr[sub_arr_idx])
        {
            sub_arr_idx = sub_arr_idx + 1;
        }
        org_arr_idx = org_arr_idx + 1;
    }
    if sub_arr_idx == sub_arr.len()
    {
        println!("SubArray");
    }
    else 
    {
        println!("Not SubArray")
    }
}