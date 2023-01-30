fn main() {
    println!("while");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // as long as pop return some
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    println!("for");
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} - {}", index, value);
    }

    println!("let");
    let (x, y, z) = (1, 2, 3);
    println!("{} - {} - {}", x, y, z);
}
