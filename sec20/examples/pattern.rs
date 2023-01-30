fn main() {
    // Refutable
    let value: Option<i32> = None;
    // let Some(x) = value;
    // `let` bindings require an "irrefutable pattern",
    //  like a `struct` or an `enum` with only one variant
    let x = if let Some(x) = value { Some(x) } else { None };
    println!("{:?}", x)

    // Irrefutable
}
