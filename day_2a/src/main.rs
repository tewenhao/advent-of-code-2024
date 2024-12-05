use std::fs;
use std::str;

// for parameters, need to be specific with their types
// this is how to pass an immutable slice as a parameter
fn is_safe(numbers: &[i32]) -> bool {
    let is_asc: bool = numbers[0] < numbers[1];
    const MINDIFF: i32 = 1;
    const MAXDIFF: i32 = 3;

    for i in 1..numbers.len() {
        let violate_diff: bool = (numbers[i - 1] - numbers[i]).abs() < MINDIFF || (numbers[i - 1] - numbers[i]).abs() > MAXDIFF;
        let violate_dir: bool = (numbers[i - 1] < numbers[i]) != is_asc;

        if violate_diff || violate_dir {
            return false;
        }
    }

    true
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