# Day 1 Part 1

## Thought Process

Quite a straightforward problem:

1. Read each line
2. Get the two numbers out
3. First number stored in one list, second number stored in another
4. Sort both lists in ascending order after all lines are read
5. Take the absolute value of the difference of the two values from the first and second lists *at the same index*

Anyway final answer is `1580061`.

## Reflections

What was challenging for me was to translate my thought process above into Rust, since this is my first time coding in Rust.

My main takeaways were:

1. how to read files and split lines:

    ```rust
    let contents: String = fs::read_to_string("input.txt")
            .expect("Should have been able to read the input file");
    let lines: str::Split<'_, char> = contents.split('\n');
    ```

2. empty vectors can be declared as such:

    ```rust
    let mut left_list: Vec<i32> = vec![];
    ```

3. splitting by whitespace to get each part of the string:

    ```rust
    let nums_str: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
    ```

4. (non technical but) how to NOT GIVE UP!!! i spent so long trying to do this and i almost gave up after 3 hours and defaulting back to Python (because i am comfortable with python) BUT YES I DID NOT GIVE UP!!!
