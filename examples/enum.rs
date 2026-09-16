// NOTE:
// Rust is already better than golang
enum Direction {
    North,
    East,
    South,
    West,
}

// NOTES:
// Associated values attached to enums,
// so they can actually be initated (haven't seen
// a better syntax feature before)
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    // NOTES:
    // Access values of enums using pattern matching
    let area = match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(r) => r * r,
    };
    return area;
}

fn main() {
    let my_direction = Direction::North;
    let new_direction = my_direction;
    move_around(new_direction);

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(5.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
    calculate_area(circle);
    calculate_area(square);
    calculate_area(rectangle);
}

fn move_around(direction: Direction) {
    // NOTES:
    // Doesn't work because not declared as mutable
    // direction = Direction::South;

    // NOTES:
    // Also doesn't work because no associated
    // print/string implemented for it
    // println!("{}", direction);
}
