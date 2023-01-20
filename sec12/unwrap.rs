use std::fs::File;
fn main() {
    // let f = File::open("hello2.txt").unwrap();
    let f = File::open("hello2.txt").expect("Failed");
}
