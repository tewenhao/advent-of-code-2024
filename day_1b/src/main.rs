use std::fs;
use std::str;
use std::collections::HashMap;

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

    let mut right_hashmap: HashMap<i32, i32> = HashMap::new();

    for num in right_list {
        let count = right_hashmap.entry(num).or_insert(0);
        *count += 1;
    }

    for num in left_list {
        let count = right_hashmap.get(&num).copied().unwrap_or(0);
        total += num * count;
    }

    println!("Final similarity score is {total}");
}