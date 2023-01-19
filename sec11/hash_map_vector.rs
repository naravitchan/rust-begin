use std::collections::HashMap;
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    // let mut team: Vec<String> = Vec::new();
    // let mut score: Vec<String> = Vec::new();
    let team = vec![String::from("Blue"), String::from("Red")];
    let score = vec![1, 2];
    let scores: HashMap<_, _> = team.iter().zip(score.iter()).collect();
    print_type_of(&scores);
    println!("{:?}", scores);

    // let mut scores = HashMap::new();
    // scores.insert("Blue", 10);
    // scores.insert("Red", 20);
    // scores.entry("Red").or_insert(30);
    let r = String::from("Red");
    let score = scores.get(&r);
    print_type_of(&scores);
    println!("{:?}", score);
}
