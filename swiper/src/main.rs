struct Point<T> {
    x: T,
    y: T
}

enum Color {
    Red,
    Green,
    Blue
}

fn main () {
    let point = Point { x: 1, y: 2 };
    let point2 = Point { x: 3, y: 4 };
    let point3 = Point { x: 5, y: 6 };
    let point4 = Point { x: 7, y: 8 };
    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;

    let points: [Point<i32>; 3] = [point, point2, point3];
    let colors: [Color; 3] = [red, green, blue];
}