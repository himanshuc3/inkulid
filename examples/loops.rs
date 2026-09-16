fn main() {
    // NOTES:
    // Not a function but a macro
    println!("{}", fib(10))
}

// NOTE:
// i32 -> signed int; u32 -> unsigned 32 bit integer
fn fib(num: i32) -> u32 {
    // NOTES:
    // by default variables are const
    // which would need some getting used to
    // Also, how awesome is implicit typing and
    // it's language server to show what type has been
    // assigned to it.
    let mut first = 0;
    let mut second = 1;
    if num == 0 {
        return first;
    }
    if num == 1 {
        return 1;
    }

    // NOTES:
    // Excludes the upperbound
    for _ in 1..num - 2 {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}
