fn main() {
    // panic!("hey ");
    // let a = [1, 2, 3];
    // a[99]; // thread 'main' panicked at 'hey ', panic.rs:2:5

    let choice = "s";
    println!("whats your choice");
    if choice == "buy" {
        println!("Thx");
    } else {
        panic!("Pls buy it");
    }
}
 