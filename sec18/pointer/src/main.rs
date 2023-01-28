use List::{Cons, Nil};
mod my_box;

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Hello, world!");

    // Dereference
    let x = 5;
    // let y = &x; error
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
