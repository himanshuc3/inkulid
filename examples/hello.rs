// NOTES:
// cargo init - for initializing an executable project
// cargo run --example [filename]
// Syntax is very relatable to golang, is it also
// another anti OOPs language
fn main() {
    // NOTES:
    // Not a function but a macro
    println!("{}", is_even(17))
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
