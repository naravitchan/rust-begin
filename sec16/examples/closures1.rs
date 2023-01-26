use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let expensive_closure = |num| {
        println!("Calculation Slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // let x = expensive_closure(2);
    let x = expensive_closure(2.2);
    println!("{}", x);

    // fn add_one_v1(x: u32) -> u32 {
    //     x + 1
    // }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v4 = |x| x + 1;
    // let add_one_v5 = |x| x + 1;
}
