# Day 1 Part 2

## Thought Process

Again, quite a straightforward problem:

1. Read each line
2. Get the two numbers out
3. First number stored in one list, second number stored in a hashmap
4. For each element in the list, add the result of it multiplied to the occurrence stored in the hashmap to the final sum

Anyway final answer is `23046913`.

## Reflections

Again, my unfamiliarity with Rust proved to be the biggest obstacle to this problem.

My main takeaways were:

1. declaring a new hashmap:

    ```rust
    let mut right_hashmap: HashMap<i32, i32> = HashMap::new();
    ```

2. value increment:

    ```rust
    let count = right_hashmap.entry(num).or_insert(0);
    *count += 1;
    ```

3. get value:

    ```rust
    let count = right_hashmap.get(&num).copied().unwrap_or(0);
    ```
