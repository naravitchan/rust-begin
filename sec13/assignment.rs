extern crate num; // cargo
use num::Num;
struct S<T: Num> {
    a: T,
    b: T,
}
impl<T: Num> S<T> {
    fn add(self) -> T {
        self.a + self.b
    }
}

fn main() {
    let s1 = S { a: 1, b: 2 };
    println!("{}", s1.add());
}
