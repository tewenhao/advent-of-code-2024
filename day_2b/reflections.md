# Day 2 Part 2

## Thought Process

A lot more nuanced compared to Part 1, so the approach in Part 1 can't be extended that simply:

1. Read each line
2. Get all the numbers in the line
3. If length of sequence is 2 or less, it automatically passes
4. Traverse through the numbers and check each pair of adjacent numbers until a violation
5. If this is NOT the first violation, early termination and return `false`
6. If upon removing either the first or second number in the pair the sequence becomes safe (for now), then continue to check the other numbers. If not early termination and return `false`
7. If all the numbers pass, then return `true`!

Anyway final answer is `436`.

### Additional Considerations in my approach

- Rather than use the first 4 elements to determine whether the sequence was increasing or decreasing, I realised that an edge case of all first 4 elements being equal will break the check. I therefore opted to set ascending/descending orders as a boolean parameter in the function.

- I was also not careful initially and only considered removing the second number (ie. for first -> second, only remove second), when the first number could also be removed and should be checked if the report could still be safe.

- I was also dumb to add an extra increment to `idx` within the `if violation` block, thinking that after the `if` block was executed, it will simply `continue` to the next cycle of the loop (news flash: I did not use `else`...) so this led me on a wild goose chase as I tried to understand what went wrong with my code.

## Reflections

I was stuck on this problem for VERY LONG. A few days actually.

Anyway, here are my main takeaways:

1. When I used the original method of splitting lines using `contents.split('\n')` and then splitting each of the numbers, there is a `\r` appended at the end of the last nuimber, which was VERY annoying. Instead, this is what I did to split lines:

    ```rust
    let strings: Vec<&str> = report.split(' ').collect();
    let numbers: Vec<i32> = strings.iter().flat_map(|x| x.parse()).collect();
    ```

2. How to output the parsed numbers (more for debugging lol):

    ```rust
    println!("Parsed numbers: {:?}", strings);
    ```
