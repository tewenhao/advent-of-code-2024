use regex::Regex;
use std::fs;

fn main() {
    // my variable names here follow the regex documentation cuz im new to rust regex.
    // well im new to regex in general
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let haystack: String = fs::read_to_string("input.txt")
        .expect("Should have been able to read the input.");
    let valid_muls: Vec<&str> = re.find_iter(&haystack).map(|m| m.as_str()).collect();

    let mut total: i32 = 0;

    for mul in valid_muls {
        let inner_re = Regex::new(r"[0-9]+").unwrap();
        let numbers: Vec<&str> = inner_re.find_iter(&mul).map(|m| m.as_str()).collect();
        total += numbers[0].parse::<i32>().unwrap() * numbers[1].parse::<i32>().unwrap();
    }

    println!("the sum of all the multiplications is {}!", total);
}
