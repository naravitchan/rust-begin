fn main() {
    println!("{:?}", factorial(10));
}

fn factorial(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    return n * factorial(n - 1);
}
