use std::fs;
use std::str;

fn is_safe_asc(numbers: &[i32]) -> bool {
    for i in 1..numbers.len() {
        if numbers[i] - numbers[i - 1] < 1 || numbers[i] - numbers[i - 1] > 3 {
            return false;
        }
    }

    true
}

fn is_safe_desc(numbers: &[i32]) -> bool {
    for i in 1..numbers.len() {
        if numbers[i - 1] - numbers[i] < 1 || numbers[i - 1] - numbers[i] > 3 {
            return false;
        }
    }

    true
}

// for parameters, need to be specific with their types
// this is how to pass an immutable slice as a parameter
fn is_safe(numbers: &[i32]) -> bool {
    is_safe_asc(numbers) || is_safe_desc(numbers)
}

fn main() {
    let contents: String = fs::read_to_string("input.txt")
        .expect("Should have been able to read the input file");
    let reports: str::Split<'_, char> = contents.split('\n');
    let mut safe_count = 0;

    for report in reports {
        if !report.is_empty() {    // account for the empty line at the end
            let strings: Vec<&str> = report.split(' ').collect();
            let numbers: Vec<i32> = strings.iter().flat_map(|x| x.parse()).collect();
    
            safe_count += is_safe(&numbers) as i32;
        }   
    }

    println!("{}", safe_count);
}