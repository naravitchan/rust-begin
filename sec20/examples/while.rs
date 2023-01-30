fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // as long as pop return some
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
