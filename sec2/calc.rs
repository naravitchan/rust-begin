use std::io;
fn main() {
    let mut o = String::new();
    let a = 10;
    let b = 2;
    println!("Choose operation to be performed");
    println!("1. +\n2. -\n3. *\n4. /");
    io::stdin().read_line(&mut o).expect("Failed");
    o = o.trim().to_string();

    if o == "+" {
        println!("{}", a + b);
    } else if o == "-" {
        println!("{}", a - b);
    } else if o == "*" {
        println!("{}", a * b);
    } else if o == "/" {
        println!("{}", a / b);
    } else {
        println!("Wrong Choice");
    }
}
