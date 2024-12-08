use std::fs;
use std::str;

/**
 * first time offence: need to check whether to remove current number or next number
 * edge case: first 2 numbers flag out error
 * edge case: last 2 numbers flag out error
 * 
 * while working on how to resolve the edge cases, i felt the best way was probs to fix the direction first
 * since the case of 1 5 4 3 2 1 would break the "setting direction as numbers[0] < numbers[1]" thing
*/
fn is_safe(numbers: &[i32], is_asc: bool) -> bool {
    // 2 numbers or less is always true
    if numbers.len() < 3 {
        return true;
    }

    const MINDIFF: i32 = 1;
    const MAXDIFF: i32 = 3;

    let mut idx: usize = 1;
    let mut dampener_used: bool = false;

    while idx < numbers.len() {
        let violate_dir: bool = (numbers[idx - 1] < numbers[idx]) != is_asc;
        let violate_diff: bool = (numbers[idx - 1] - numbers[idx]).abs() < MINDIFF || (numbers[idx - 1] - numbers[idx]).abs() > MAXDIFF;

        // let violation be defined as first -> second
        if violate_diff || violate_dir {
            if dampener_used {
                return false;
            }

            dampener_used = true;

            // case: remove second element

            // if second element is the last element in the vector, then just skip
            if idx == numbers.len() - 1 {
                break;
            }

            // removing second element
            let remove_second_violate_dir: bool = (numbers[idx - 1] < numbers[idx + 1]) != is_asc;
            let remove_second_violate_diff: bool = (numbers[idx - 1] - numbers[idx + 1]).abs() < MINDIFF || (numbers[idx - 1] - numbers[idx + 1]).abs() > MAXDIFF;

            if (!remove_second_violate_diff) && (!remove_second_violate_dir) {
                idx += 2;
                continue;
            }

            // case: remove first element
            if idx > 1 {
                // compare element from 2 spaces before
                let remove_first_violate_dir: bool = (numbers[idx - 2] < numbers[idx]) != is_asc;
                let remove_first_violate_diff: bool = (numbers[idx - 2] - numbers[idx]).abs() < MINDIFF || (numbers[idx - 2] - numbers[idx]).abs() > MAXDIFF;

                // the reason why we only consider this case is because considering x -> first -> second -> third,
                // if first is removed, then x -> second must be valid. if it is still not valid, cannot be salvaged
                // second -> third will be checked later with an increment of idx
                if remove_first_violate_diff || remove_first_violate_dir {
                    return false;
                }
            }
        }
        
        idx += 1;
    }

    true
}

fn main() {
    let contents: String = fs::read_to_string("input.txt")
        .expect("Should have been able to read the input file");
    let reports: Vec<String> = contents.lines()
        .map(|line| line.trim().to_string())
        .collect();
    let mut safe_count = 0;

    for report in reports {
        if !report.is_empty() {
            let strings: Vec<&str> = report.split(' ').collect();
            let numbers: Vec<i32> = strings.iter().flat_map(|x| x.parse()).collect();

            // println!("Raw input: {}", report);
            // println!("Parsed numbers: {:?}", strings);
            // println!("Ascending check: {}", is_safe(&numbers, true));
            // println!("Descending check: {}", is_safe(&numbers, false));

            safe_count += (is_safe(&numbers, true) || is_safe(&numbers, false)) as i32;
        }   
    }

    println!("{}", safe_count);
}