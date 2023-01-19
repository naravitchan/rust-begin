use std::collections::HashMap;

fn main() {
    let mut score = HashMap::new();
    score.insert("Blue", 10);
    score.insert("Red", 20);
    println!("{:?}", score);
}
