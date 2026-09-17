// Ownership, moving,
// borrowing, references

// Different techniques of memory management:
// 1. Garbage collector
// 2. Manual: allocate and deallocate memory yourself
// 3. The rust way: RAII pattern

// Ownership: Each value will have it's owner
fn create_string() {
    let romaji_intro = String::from("Watashi wa himanshu desu");

    println!("My first sentence was: {}", romaji_intro);
}

fn create_string_2() {
    let s1 = String::from("Konnichiwa");
    let s2 = s1;

    // Error: s2 has the ownership now, since we moved s1 to s2
    // Might feel very stupid, given references are readily
    // used passing around pointers, so at any given time
    // multiple variables are pointing to the same data.
    // println!("{}", s1);
    println!("{}", s2);
}

fn sleep(s2: String) -> String {
    println!("{}", s2);
    return s2;
}

fn sleep_again(s2: &String) {
    println!("{}", s2);
}

// At one time we can have 1 mutable reference
// or n immutable references. Almost like dirty read locks.
fn main() {
    let mut status = String::from("idle");
    let mut status2 = String::from("progress");
    create_string();
    create_string_2();

    // Moved back the ownership
    status = sleep(status);
    // Lending to sleep_again whereas the ownership is retained
    sleep_again(&status2);

    println!("{}", status);
    println!("{}", status2);
}
