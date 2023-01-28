enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use std::rc::Rc;
use List::{Cons, Nil};

// cargo run --example reference_count
fn main() {
    println!("Hello, world! ref_count");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Box::new(a)); // - value moved here
    // let c = Cons(4, Box::new(a)); // ^ value used here after move
    println!("counter after creating a {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("counter after creating b {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("counter after  creating c {}", Rc::strong_count(&a));
    }
    println!("counter after dropping  c {}", Rc::strong_count(&a));
}
