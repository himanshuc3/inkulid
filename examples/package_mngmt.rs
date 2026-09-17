// cargo add [caret_name]
use chrono::{Local, Utc};

// Memort management:
// Sotred on the stack: numbers, booleans, fixed size arrays, references
// Stored on the heap: strings, vectors, hashmap
fn main() {
    let now = Local::now();
    println!("current time is {}", now);
}
