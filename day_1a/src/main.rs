use std::fs;
use std::str;

fn main() {
    let contents: String = fs::read_to_string("input.txt")
        .expect("Should have been able to read the input file");
    let lines: str::Split<'_, char> = contents.split('\n');

    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    let mut total: i32 = 0;

    for line in lines {
        if !line.is_empty() {
            let nums_str: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
            let (nums_str1, nums_str2) = (nums_str[0], nums_str[1]);

            // i can't just do `let nums_str1: i32 = nums_str1.parse::<i32>().unwrap();` since rust typing is enforced
            let nums1: i32 = nums_str1.parse::<i32>().unwrap();
            let nums2: i32 = nums_str2.parse::<i32>().unwrap();

            left_list.push(nums1);
            right_list.push(nums2);
        }
    }

    left_list.sort();
    right_list.sort();

    for i in 0..left_list.len() {
        let val: i32 = left_list[i] - right_list[i];
        total += val.abs();
    }

    println!("Total distance between the two lists is {total}");
}