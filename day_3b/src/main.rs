use regex::Regex;
use std::fs;

/// **Dumping thought processes here**
/// * current approach is to get ALL the muls first and evaluate, then get all the muls between don't() and do() and evaluate the subtract
/// * i think i need to make my regex matches non greedy/lazy if not the don't() do() boundary won't work;
/// see https://stackoverflow.com/questions/2503413/regular-expression-to-stop-at-first-match
/// * i also need to account for split lines? since the don't() command doesn't carry over to the next line since it's "broken"
/// so i need to join the lines together
fn main() {
    // since input data is the same, we use the answer we got from part 1 as the "ALL the muls evaluated"
    // const  TEST_INPUT_MULS_TOTAL: i32 = 204;
    const ACTUAL_INPUT_MULS_TOTAL: i32 = 184122457;

    let re = Regex::new(r"don't\(\)(.*?)do\(\)").unwrap();
    let haystack: String = fs::read_to_string("input.txt")
        .expect("Should have been able to read the input.")
        + "do()";
    let haystack: &str = &haystack.replace('\n', "").replace('\r', "");

    let matches: Vec<&str> = re.find_iter(&haystack).map(|m| m.as_str()).collect();

    let mut total: i32 = 0;

    for i in matches {
        // extract out all the muls
        let re_mul = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
        let muls: Vec<&str> = re_mul.find_iter(i).map(|m| m.as_str()).collect();

        for mul in muls {
            let inner_re = Regex::new(r"[0-9]+").unwrap();
            let numbers: Vec<&str> = inner_re.find_iter(&mul).map(|m| m.as_str()).collect();
            total += numbers[0].parse::<i32>().unwrap() * numbers[1].parse::<i32>().unwrap();
        }
    }

    println!("After deducting {} from ALL mul commands, the answer we seek is {}", total, ACTUAL_INPUT_MULS_TOTAL - total);
}