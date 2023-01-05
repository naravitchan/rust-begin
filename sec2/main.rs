fn main() {
    let a: i32 = 10;
    let a: i64 = a as i64 + 10;
    println!("{}", a);

    let a = 10;
    if a % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
