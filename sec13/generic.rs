#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn number(&self) -> f32 {
        self.x
    }
}

impl Point<i32> {
    fn number(&self) -> i32 {
        self.y
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 8.0, y: 9.1 };
    println!("{:?}\n{:?}", integer.x(), float);
    println!("{:?}", float.number());
    println!("{:?}", integer.number());
}
