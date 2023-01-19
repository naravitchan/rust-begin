fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    println!("{:?}", v);
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);

    // let mut v = Vec::new(); err  type annotations needed for `Vec<T>`

    // let mut v = vec![1, "2", 3]; err expected integer, found `&str`
}
