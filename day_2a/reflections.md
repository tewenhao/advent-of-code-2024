# Day 2 Part 1

## Thought Process

Not as straightforward as day 1, but still managable:

1. Read each line
2. Get all the numbers in the line
3. Check if any two adjacent levels break the rules
4. If none of the adjacent levels break the rules, increase counter by 1

Anyway final answer is `383`.

## Reflections

Not being familiar with Rust really made things difficult...

My main takeaways were:

1. parse each of the numbers (which are strings) to integers

    ```rust
    let strings: Vec<&str> = report.split(' ').collect();
    let numbers: Vec<i32> = strings.iter().flat_map(|x| x.parse()).collect();
    ```

2. function parameters must have their types declared upfront:

    ```rust
    fn is_safe(numbers: &[i32]) -> bool {
        ...
    }
    ```

3. how to pass an immutable slice as a parameter:

    ```rust
    fn is_safe(numbers: &[i32]) -> bool {
        ...
    }
    ```
