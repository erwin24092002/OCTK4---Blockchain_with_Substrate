/*
Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
let sub_arr = [6,8,10];
 */
#![allow(unused)]
fn main() {
    let org_arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sub_arr: [i32; 3] = [4, 6, 7];

    for org_arr_item in org_arr 
    {
        let mut flag = true;
        for sub_arr_item in sub_arr 
        {
            if (sub_arr_item == org_arr_item)
            {
                flag = true;
                break;
            }
        }
    }
    println!("Subarray");
}