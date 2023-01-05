fn main() {
    let a: (i32, bool, f64) = (200, true, 8.3);
    print(a);
}

fn print(x: (i32, bool, f64)) {
    let (a, y, z) = x;
    println!("{} {} {}", a, y, z);
}
