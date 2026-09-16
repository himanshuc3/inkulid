// NOTES:
// usize is a subset of i32 i assume
fn get_string_length(s: String) -> usize {
    // NOTES:
    // Implicit returns is already the most useless
    // feature of the language for me
    s.chars().count()
}

fn main() {
    let my_string = String::from("hello, world!");
    let length = get_string_length(my_string);
    println!("The length of the string is {}", length)
}
