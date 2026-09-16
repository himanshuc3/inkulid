// NOTES:
// The differences with go would be
// that it doesn't have a zero value here
// and structs are more closer to classes
struct User {
    active: bool,
    username: String,
    age: i32,
}

struct Rect {
    width: u32,
    height: u32,
}

// NOTES:
// Defining behavior on user defined data type struct
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // NOTE:
    // We have static functions as well, hurayyyyy
    fn debug() -> &'static str {
        let value = "This is a static function";
        return value;
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("bobity"),
        age: 69,
    };

    let rect = Rect {
        width: 30,
        height: 33,
    };
    print!("The area of the rectangle is {}\n", rect.area());
    println!("{}", Rect::debug());

    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{}", user1.age);
}
