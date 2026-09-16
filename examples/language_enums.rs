use std::fs;
// NOTES:
// Options enum enables returning none/nul/nil
// pub enum Option<T> {
//     None,
//     Some(T),
// }
fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

// NOTES:
// Result enum is how we do error handling
// in rust. The naming convention could be improved
// to far superior and already trusted camelcase though
// Result and Options use generics to support different types
// instead of implementing it bound to a type
fn read_file_contents() {
    let greeting_file_result = fs::read_to_string("hello.txt");

    // NOTE:
    // Similar to how golang treat errors as values instead of
    // exception handling
    // We have Ok(), Err() and panic!() constructs for returning a Result or
    // panicing similar to golang
    match greeting_file_result {
        Ok(file_content) => {
            println!("File read success: {:?}", file_content);
        }
        Err(error) => {
            println!("Failed to read file: {:?}", error)
        }
    }
}

fn main() {
    let index = find_first_a(String::from("preet"));
    read_file_contents();
    // NOTE:
    // Somehow feels better version of case
    match index {
        Some(value) => println!("index is {}", value),
        None => println!("a not found"),
    }
}
